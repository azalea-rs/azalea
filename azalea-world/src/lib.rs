#![feature(int_roundings)]

mod bit_storage;
mod chunk;
mod entity;
mod palette;

use azalea_block::BlockState;
use azalea_buf::BufReadError;
use azalea_core::{BlockPos, ChunkPos, EntityPos, PositionDelta8};
use azalea_entity::Entity;
pub use bit_storage::BitStorage;
pub use chunk::{Chunk, ChunkStorage};
pub use entity::EntityStorage;
use std::{
    io::Read,
    ops::{Index, IndexMut},
    sync::{Arc, Mutex},
};
use thiserror::Error;
use uuid::Uuid;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

/// A dimension is a collection of chunks and entities.
#[derive(Debug)]
pub struct Dimension {
    chunk_storage: ChunkStorage,
    entity_storage: EntityStorage,
}

#[derive(Error, Debug)]
pub enum MoveEntityError {
    #[error("Entity doesn't exist")]
    EntityDoesNotExist,
}

impl Dimension {
    pub fn new(chunk_radius: u32, height: u32, min_y: i32) -> Self {
        Dimension {
            chunk_storage: ChunkStorage::new(chunk_radius, height, min_y),
            entity_storage: EntityStorage::new(),
        }
    }

    pub fn replace_with_packet_data(
        &mut self,
        pos: &ChunkPos,
        data: &mut impl Read,
    ) -> Result<(), BufReadError> {
        self.chunk_storage.replace_with_packet_data(pos, data)
    }

    pub fn update_view_center(&mut self, pos: &ChunkPos) {
        self.chunk_storage.view_center = *pos;
    }

    pub fn get_block_state(&self, pos: &BlockPos) -> Option<BlockState> {
        self.chunk_storage.get_block_state(pos, self.min_y())
    }

    pub fn move_entity(
        &mut self,
        entity_id: u32,
        new_pos: EntityPos,
    ) -> Result<(), MoveEntityError> {
        let entity = self
            .entity_storage
            .get_mut_by_id(entity_id)
            .ok_or(MoveEntityError::EntityDoesNotExist)?;

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

    pub fn move_entity_with_delta(
        &mut self,
        entity_id: u32,
        delta: &PositionDelta8,
    ) -> Result<(), MoveEntityError> {
        let entity = self
            .entity_storage
            .get_mut_by_id(entity_id)
            .ok_or(MoveEntityError::EntityDoesNotExist)?;
        let new_pos = entity.pos().with_delta(delta);

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

    pub fn mut_entity_by_id(&mut self, id: u32) -> Option<&mut Entity> {
        self.entity_storage.get_mut_by_id(id)
    }

    pub fn entity_by_uuid(&self, uuid: &Uuid) -> Option<&Entity> {
        self.entity_storage.get_by_uuid(uuid)
    }

    /// Get an iterator over all entities.
    #[inline]
    pub fn entities(&self) -> std::collections::hash_map::Values<'_, u32, Entity> {
        self.entity_storage.entities()
    }

    pub fn find_one_entity<F>(&self, mut f: F) -> Option<&Entity>
    where
        F: FnMut(&Entity) -> bool,
    {
        self.entity_storage.find_one_entity(|entity| f(entity))
    }
}

impl Index<&ChunkPos> for Dimension {
    type Output = Option<Arc<Mutex<Chunk>>>;

    fn index(&self, pos: &ChunkPos) -> &Self::Output {
        &self.chunk_storage[pos]
    }
}
impl IndexMut<&ChunkPos> for Dimension {
    fn index_mut<'a>(&'a mut self, pos: &ChunkPos) -> &'a mut Self::Output {
        &mut self.chunk_storage[pos]
    }
}
