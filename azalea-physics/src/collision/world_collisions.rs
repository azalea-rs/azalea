use std::sync::Arc;

use azalea_block::BlockState;
use azalea_core::{
    cursor3d::{Cursor3d, CursorIterationType},
    math::EPSILON,
    position::{BlockPos, ChunkBlockPos, ChunkPos, ChunkSectionBlockPos, ChunkSectionPos},
};
use azalea_world::{Chunk, Instance};
use parking_lot::RwLock;

use super::{Shapes, BLOCK_SHAPE};
use crate::collision::{BlockWithShape, VoxelShape, AABB};

pub fn get_block_collisions(world: &Instance, aabb: AABB) -> Vec<VoxelShape> {
    let mut state = BlockCollisionsState::new(world, aabb);
    let mut block_collisions = Vec::new();

    let initial_chunk_pos = ChunkPos::from(state.cursor.origin());
    let initial_chunk = world.chunks.get(&initial_chunk_pos);
    let initial_chunk = initial_chunk.as_deref().map(RwLock::read);

    while let Some(item) = state.cursor.next() {
        if item.iteration_type == CursorIterationType::Corner {
            continue;
        }

        let item_chunk_pos = ChunkPos::from(item.pos);
        let block_state: BlockState = if item_chunk_pos == initial_chunk_pos {
            if let Some(initial_chunk) = &initial_chunk {
                initial_chunk
                    .get(&ChunkBlockPos::from(item.pos), state.world.chunks.min_y)
                    .unwrap_or(BlockState::AIR)
            } else {
                BlockState::AIR
            }
        } else {
            state.get_block_state(item.pos)
        };

        if block_state.is_air() {
            // fast path since we can't collide with air
            continue;
        }

        // TODO: continue if self.only_suffocating_blocks and the block is not
        // suffocating

        // if it's a full block do a faster collision check
        if block_state.is_shape_full() {
            if !state.aabb.intersects_aabb(&AABB {
                min_x: item.pos.x as f64,
                min_y: item.pos.y as f64,
                min_z: item.pos.z as f64,
                max_x: (item.pos.x + 1) as f64,
                max_y: (item.pos.y + 1) as f64,
                max_z: (item.pos.z + 1) as f64,
            }) {
                continue;
            }

            block_collisions.push(BLOCK_SHAPE.move_relative(
                item.pos.x as f64,
                item.pos.y as f64,
                item.pos.z as f64,
            ));
            continue;
        }

        let block_shape = state.get_block_shape(block_state);

        let block_shape =
            block_shape.move_relative(item.pos.x as f64, item.pos.y as f64, item.pos.z as f64);
        // if the entity shape and block shape don't collide, continue
        if !Shapes::matches_anywhere(&block_shape, &state.entity_shape, |a, b| a && b) {
            continue;
        }

        block_collisions.push(block_shape);
    }

    block_collisions
}

pub struct BlockCollisionsState<'a> {
    pub world: &'a Instance,
    pub aabb: AABB,
    pub entity_shape: VoxelShape,
    pub cursor: Cursor3d,

    cached_sections: Vec<(ChunkSectionPos, azalea_world::Section)>,
    cached_block_shapes: Vec<(BlockState, &'static VoxelShape)>,
}

impl<'a> BlockCollisionsState<'a> {
    pub fn new(world: &'a Instance, aabb: AABB) -> Self {
        let origin = BlockPos {
            x: (aabb.min_x - EPSILON).floor() as i32 - 1,
            y: (aabb.min_y - EPSILON).floor() as i32 - 1,
            z: (aabb.min_z - EPSILON).floor() as i32 - 1,
        };

        let end = BlockPos {
            x: (aabb.max_x + EPSILON).floor() as i32 + 1,
            y: (aabb.max_y + EPSILON).floor() as i32 + 1,
            z: (aabb.max_z + EPSILON).floor() as i32 + 1,
        };

        let cursor = Cursor3d::new(origin, end);

        Self {
            world,
            aabb,
            entity_shape: VoxelShape::from(aabb),
            cursor,

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
                return cached_section.get(section_block_pos);
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

        section.get(section_block_pos)
    }

    fn get_block_shape(&mut self, block_state: BlockState) -> &'static VoxelShape {
        for (cached_block_state, cached_shape) in &self.cached_block_shapes {
            if block_state == *cached_block_state {
                return cached_shape;
            }
        }

        let shape = block_state.shape();
        self.cached_block_shapes.push((block_state, shape));

        shape
    }
}
