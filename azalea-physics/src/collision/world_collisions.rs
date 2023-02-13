use super::Shapes;
use crate::collision::{BlockWithShape, VoxelShape, AABB};
use azalea_block::BlockState;
use azalea_core::{ChunkPos, ChunkSectionPos, Cursor3d, CursorIterationType, EPSILON};
use azalea_world::{Chunk, World};
use parking_lot::RwLock;
use std::sync::Arc;

pub fn get_block_collisions(world: &World, aabb: AABB) -> BlockCollisions<'_> {
    BlockCollisions::new(world, aabb)
}

pub struct BlockCollisions<'a> {
    pub world: &'a World,
    pub aabb: AABB,
    pub entity_shape: VoxelShape,
    pub cursor: Cursor3d,
    pub only_suffocating_blocks: bool,
}

impl<'a> BlockCollisions<'a> {
    pub fn new(world: &'a World, aabb: AABB) -> Self {
        let origin_x = (aabb.min_x - EPSILON) as i32 - 1;
        let origin_y = (aabb.min_y - EPSILON) as i32 - 1;
        let origin_z = (aabb.min_z - EPSILON) as i32 - 1;

        let end_x = (aabb.max_x + EPSILON) as i32 + 1;
        let end_y = (aabb.max_y + EPSILON) as i32 + 1;
        let end_z = (aabb.max_z + EPSILON) as i32 + 1;

        let cursor = Cursor3d::new(origin_x, origin_y, origin_z, end_x, end_y, end_z);

        Self {
            world,
            aabb,
            entity_shape: VoxelShape::from(aabb),
            cursor,
            only_suffocating_blocks: false,
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
}

impl<'a> Iterator for BlockCollisions<'a> {
    type Item = VoxelShape;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.cursor.next() {
            if item.iteration_type == CursorIterationType::Corner {
                continue;
            }

            let chunk = self.get_chunk(item.pos.x, item.pos.z);
            let Some(chunk) = chunk else {
                continue
            };

            let pos = item.pos;
            let block_state: BlockState = chunk
                .read()
                .get(&(&pos).into(), self.world.chunks.min_y)
                .unwrap_or(BlockState::AIR);

            // TODO: continue if self.only_suffocating_blocks and the block is not
            // suffocating

            let block_shape = block_state.shape();

            // if it's a full block do a faster collision check
            if block_shape == &crate::collision::block_shape() {
                if !self.aabb.intersects_aabb(&AABB {
                    min_x: item.pos.x as f64,
                    min_y: item.pos.y as f64,
                    min_z: item.pos.z as f64,
                    max_x: (item.pos.x + 1) as f64,
                    max_y: (item.pos.y + 1) as f64,
                    max_z: (item.pos.z + 1) as f64,
                }) {
                    continue;
                }

                return Some(block_shape.move_relative(
                    item.pos.x as f64,
                    item.pos.y as f64,
                    item.pos.z as f64,
                ));
            }

            let block_shape =
                block_shape.move_relative(item.pos.x as f64, item.pos.y as f64, item.pos.z as f64);
            // if the entity shape and block shape don't collide, continue
            if !Shapes::matches_anywhere(&block_shape, &self.entity_shape, |a, b| a && b) {
                continue;
            }

            return Some(block_shape);
        }

        None
    }
}
