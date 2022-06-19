#![feature(int_roundings)]

mod bit_storage;
mod chunk;
mod entity;
mod palette;

use azalea_block::BlockState;
use azalea_core::{BlockPos, ChunkBlockPos, ChunkPos, ChunkSectionBlockPos, EntityPos};
use azalea_entity::Entity;
use azalea_protocol::mc_buf::{McBufReadable, McBufWritable};
pub use bit_storage::BitStorage;
pub use chunk::{Chunk, ChunkStorage};
pub use entity::EntityStorage;
use std::{
    io::{Read, Write},
    ops::{Index, IndexMut},
    sync::{Arc, Mutex},
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[derive(Debug)]
pub struct World {
    chunk_storage: ChunkStorage,
    entity_storage: EntityStorage,
}

impl World {
    pub fn new(chunk_radius: u32, height: u32, min_y: i32) -> Self {
        World {
            chunk_storage: ChunkStorage::new(chunk_radius, height, min_y),
            entity_storage: EntityStorage::new(),
        }
    }

    pub fn replace_with_packet_data(
        &mut self,
        pos: &ChunkPos,
        data: &mut impl Read,
    ) -> Result<(), String> {
        self.chunk_storage.replace_with_packet_data(pos, data)
    }

    pub fn update_view_center(&mut self, pos: &ChunkPos) {
        self.chunk_storage.view_center = *pos;
    }

    pub fn get_block_state(&self, pos: &BlockPos) -> Option<BlockState> {
        self.chunk_storage.get_block_state(pos, self.min_y())
    }

    pub fn move_entity(&mut self, entity_id: u32, new_pos: EntityPos) -> Result<(), String> {
        let entity = self
            .entity_storage
            .get_mut_by_id(entity_id)
            .ok_or("Moving entity that doesn't exist".to_string())?;
        let old_chunk = ChunkPos::from(entity.pos());
        let new_chunk = ChunkPos::from(&new_pos);
        // this is fine because we update the chunk below
        entity.unsafe_move(new_pos);
        if old_chunk != new_chunk {
            self.entity_storage
                .update_entity_chunk(entity_id, &old_chunk, &new_chunk);
        }
        Ok(())
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entity_storage.insert(entity);
    }

    pub fn height(&self) -> u32 {
        self.chunk_storage.height
    }

    pub fn min_y(&self) -> i32 {
        self.chunk_storage.min_y
    }

    pub fn entity_by_id(&self, id: u32) -> Option<&Entity> {
        self.entity_storage.get_by_id(id)
    }
}

impl Index<&ChunkPos> for World {
    type Output = Option<Arc<Mutex<Chunk>>>;

    fn index(&self, pos: &ChunkPos) -> &Self::Output {
        &self.chunk_storage[pos]
    }
}
impl IndexMut<&ChunkPos> for World {
    fn index_mut<'a>(&'a mut self, pos: &ChunkPos) -> &'a mut Self::Output {
        &mut self.chunk_storage[pos]
    }
}
