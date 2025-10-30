use std::{collections::HashSet, sync::Arc};

use azalea_block::{BlockState, fluid_state::FluidState};
use azalea_core::{
    cursor3d::{Cursor3d, CursorIteration, CursorIterationType},
    math::EPSILON,
    position::{BlockPos, ChunkBlockPos, ChunkPos, ChunkSectionBlockPos, ChunkSectionPos, Vec3},
};
use azalea_inventory::ItemStack;
use azalea_world::{Chunk, Instance};
use bevy_ecs::entity::Entity;
use parking_lot::RwLock;

use super::{BLOCK_SHAPE, Shapes};
use crate::collision::{Aabb, BlockWithShape, VoxelShape};

pub fn get_block_collisions(world: &Instance, aabb: &Aabb) -> Vec<VoxelShape> {
    let mut state = BlockCollisionsState::new(world, aabb, EntityCollisionContext::of(None));
    let mut block_collisions = Vec::new();

    let initial_chunk_pos = ChunkPos::from(state.cursor.origin());
    let initial_chunk = world.chunks.get(&initial_chunk_pos);
    let initial_chunk = initial_chunk.as_deref().map(RwLock::read);

    while let Some(item) = state.cursor.next() {
        state.compute_next(
            item,
            &mut block_collisions,
            initial_chunk_pos,
            initial_chunk.as_deref(),
        );
    }

    block_collisions
}

pub fn get_block_and_liquid_collisions(world: &Instance, aabb: &Aabb) -> Vec<VoxelShape> {
    let mut state = BlockCollisionsState::new(
        world,
        aabb,
        EntityCollisionContext::of(None).with_include_liquids(true),
    );
    let mut block_collisions = Vec::new();

    let initial_chunk_pos = ChunkPos::from(state.cursor.origin());
    let initial_chunk = world.chunks.get(&initial_chunk_pos);
    let initial_chunk = initial_chunk.as_deref().map(RwLock::read);

    while let Some(item) = state.cursor.next() {
        state.compute_next(
            item,
            &mut block_collisions,
            initial_chunk_pos,
            initial_chunk.as_deref(),
        );
    }

    block_collisions
}

pub struct BlockCollisionsState<'a> {
    pub world: &'a Instance,
    pub aabb: &'a Aabb,
    pub entity_shape: VoxelShape,
    pub cursor: Cursor3d,

    _context: EntityCollisionContext,

    cached_sections: Vec<(ChunkSectionPos, azalea_world::Section)>,
    cached_block_shapes: Vec<(BlockState, &'static VoxelShape)>,
}

impl<'a> BlockCollisionsState<'a> {
    fn compute_next(
        &mut self,
        item: CursorIteration,
        block_collisions: &mut Vec<VoxelShape>,
        initial_chunk_pos: ChunkPos,
        initial_chunk: Option<&Chunk>,
    ) {
        if item.iteration_type == CursorIterationType::Corner {
            return;
        }

        let item_chunk_pos = ChunkPos::from(item.pos);
        let block_state: BlockState = if item_chunk_pos == initial_chunk_pos {
            match &initial_chunk {
                Some(initial_chunk) => initial_chunk
                    .get_block_state(&ChunkBlockPos::from(item.pos), self.world.chunks.min_y)
                    .unwrap_or(BlockState::AIR),
                _ => BlockState::AIR,
            }
        } else {
            self.get_block_state(item.pos)
        };

        if block_state.is_air() {
            // fast path since we can't collide with air
            return;
        }

        // TODO: continue if self.only_suffocating_blocks and the block is not
        // suffocating

        // if it's a full block do a faster collision check
        if block_state.is_collision_shape_full() {
            if !self.aabb.intersects_aabb(&Aabb {
                min: item.pos.to_vec3_floored(),
                max: (item.pos + 1).to_vec3_floored(),
            }) {
                return;
            }

            block_collisions.push(BLOCK_SHAPE.move_relative(item.pos.to_vec3_floored()));
            return;
        }

        let block_shape = self.get_block_shape(block_state);

        let block_shape = block_shape.move_relative(item.pos.to_vec3_floored());
        // if the entity shape and block shape don't collide, continue
        if !Shapes::matches_anywhere(&block_shape, &self.entity_shape, |a, b| a && b) {
            return;
        }

        block_collisions.push(block_shape);
    }

    pub fn new(world: &'a Instance, aabb: &'a Aabb, context: EntityCollisionContext) -> Self {
        let origin = BlockPos {
            x: (aabb.min.x - EPSILON).floor() as i32 - 1,
            y: (aabb.min.y - EPSILON).floor() as i32 - 1,
            z: (aabb.min.z - EPSILON).floor() as i32 - 1,
        };

        let end = BlockPos {
            x: (aabb.max.x + EPSILON).floor() as i32 + 1,
            y: (aabb.max.y + EPSILON).floor() as i32 + 1,
            z: (aabb.max.z + EPSILON).floor() as i32 + 1,
        };

        let cursor = Cursor3d::new(origin, end);

        Self {
            world,
            aabb,
            entity_shape: VoxelShape::from(aabb),
            cursor,

            _context: context,

            cached_sections: Vec::new(),
            cached_block_shapes: Vec::new(),
        }
    }

    fn get_chunk(&self, block_x: i32, block_z: i32) -> Option<Arc<RwLock<Chunk>>> {
        let chunk_x = ChunkSectionPos::block_to_section_coord(block_x);
        let chunk_z = ChunkSectionPos::block_to_section_coord(block_z);
        let chunk_pos = ChunkPos::new(chunk_x, chunk_z);

        // TODO: minecraft caches chunk here
        // int chunkX = SectionPos.blockToSectionCoord(blockX);
        // int chunkZ = SectionPos.blockToSectionCoord(blockZ);
        // long chunkPosLong = ChunkPos.asLong(chunkX, chunkZ);
        // if (this.cachedBlockGetter != null && this.cachedBlockGetterPos == var5) {
        //    return this.cachedBlockGetter;
        // } else {
        //    BlockGetter var7 = this.collisionGetter.getChunkForCollisions(chunkX,
        // chunkZ);    this.cachedBlockGetter = var7;
        //    this.cachedBlockGetterPos = chunkPosLong;
        //    return var7;
        // }

        self.world.chunks.get(&chunk_pos)
    }

    fn get_block_state(&mut self, block_pos: BlockPos) -> BlockState {
        if block_pos.y < self.world.chunks.min_y {
            // below the world
            return BlockState::AIR;
        }

        let section_pos = ChunkSectionPos::from(block_pos);
        let section_block_pos = ChunkSectionBlockPos::from(block_pos);

        for (cached_section_pos, cached_section) in &self.cached_sections {
            if section_pos == *cached_section_pos {
                return cached_section.get_block_state(section_block_pos);
            }
        }

        let chunk = self.get_chunk(block_pos.x, block_pos.z);
        let Some(chunk) = chunk else {
            return BlockState::AIR;
        };
        let chunk = chunk.read();

        let sections = &chunk.sections;
        let section_index =
            azalea_world::chunk_storage::section_index(block_pos.y, self.world.chunks.min_y)
                as usize;

        let Some(section) = sections.get(section_index) else {
            return BlockState::AIR;
        };

        self.cached_sections.push((section_pos, section.clone()));

        // println!("chunk section palette: {:?}", section.states.palette);
        // println!("chunk section data: {:?}", section.states.storage.data);
        // println!("biome length: {}", section.biomes.storage.data.len());

        section.get_block_state(section_block_pos)
    }

    fn get_block_shape(&mut self, block_state: BlockState) -> &'static VoxelShape {
        for (cached_block_state, cached_shape) in &self.cached_block_shapes {
            if block_state == *cached_block_state {
                return cached_shape;
            }
        }

        let shape = block_state.collision_shape();
        self.cached_block_shapes.push((block_state, shape));

        shape
    }
}

pub struct EntityCollisionContext {
    pub descending: bool,
    pub entity_bottom: f64,
    pub held_item: ItemStack,
    can_stand_on_fluid_predicate: CanStandOnFluidPredicate,
    pub entity: Option<Entity>,
}

impl EntityCollisionContext {
    pub fn of(entity: Option<Entity>) -> Self {
        Self {
            descending: false,
            entity_bottom: 0.0,
            held_item: ItemStack::Empty,
            can_stand_on_fluid_predicate: CanStandOnFluidPredicate::PassToEntity,
            entity,
        }
    }
    pub fn with_include_liquids(mut self, include_liquids: bool) -> Self {
        self.can_stand_on_fluid_predicate = if include_liquids {
            CanStandOnFluidPredicate::AlwaysTrue
        } else {
            CanStandOnFluidPredicate::PassToEntity
        };
        self
    }

    pub fn can_stand_on_fluid(&self, above: &FluidState, target: &FluidState) -> bool {
        self.can_stand_on_fluid_predicate.matches(target) && !above.is_same_kind(target)
    }
}

enum CanStandOnFluidPredicate {
    PassToEntity,
    AlwaysTrue,
}
impl CanStandOnFluidPredicate {
    pub fn matches(&self, _state: &FluidState) -> bool {
        match self {
            Self::AlwaysTrue => true,
            // minecraft sometimes returns true for striders here, false for every other entity
            // though
            Self::PassToEntity => false,
        }
    }
}

/// This basically gets all the chunks that an entity colliding with
/// that bounding box could be in.
///
/// This is forEachAccessibleNonEmptySection in vanilla Minecraft because they
/// sort entities into sections instead of just chunks. In theory this might be
/// a performance loss for Azalea. If this ever turns out to be a bottleneck,
/// then maybe you should try having it do that instead.
pub fn for_entities_in_chunks_colliding_with(
    world: &Instance,
    aabb: &Aabb,
    mut consumer: impl FnMut(ChunkPos, &HashSet<Entity>),
) {
    let min_section = ChunkSectionPos::from(aabb.min - Vec3::new(2., 4., 2.));
    let max_section = ChunkSectionPos::from(aabb.max + Vec3::new(2., 0., 2.));

    let min_chunk = ChunkPos::from(min_section);
    let max_chunk = ChunkPos::from(max_section);

    for chunk_x in min_chunk.x..=max_chunk.x {
        for chunk_z in min_chunk.z..=max_chunk.z {
            let chunk_pos = ChunkPos::new(chunk_x, chunk_z);
            if let Some(entities) = world.entities_by_chunk.get(&chunk_pos) {
                consumer(chunk_pos, entities);
            }
        }
    }
}
