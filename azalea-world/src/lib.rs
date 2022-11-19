#![feature(int_roundings)]

mod bit_storage;
mod chunk_storage;
pub mod entity;
mod entity_storage;
mod palette;

use azalea_block::BlockState;
use azalea_buf::BufReadError;
use azalea_core::{BlockPos, ChunkPos, PositionDelta8, Vec3};
pub use bit_storage::BitStorage;
pub use chunk_storage::{Chunk, ChunkStorage};
use entity::{Entity, EntityData};
pub use entity_storage::EntityStorage;
use parking_lot::Mutex;
use std::{
    io::Cursor,
    ops::{Index, IndexMut},
    sync::Arc,
};
use thiserror::Error;
use uuid::Uuid;

/// A world is a collection of chunks and entities. They're called "levels" in Minecraft's source code.
#[derive(Debug, Default)]
pub struct World {
    pub chunk_storage: ChunkStorage,
    pub entity_storage: EntityStorage,
}

#[derive(Error, Debug)]
pub enum MoveEntityError {
    #[error("Entity doesn't exist")]
    EntityDoesNotExist,
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
        data: &mut Cursor<&[u8]>,
    ) -> Result<(), BufReadError> {
        self.chunk_storage.replace_with_packet_data(pos, data)
    }

    pub fn set_chunk(&mut self, pos: &ChunkPos, chunk: Option<Chunk>) -> Result<(), BufReadError> {
        self[pos] = chunk.map(|c| Arc::new(Mutex::new(c)));
        Ok(())
    }

    pub fn update_view_center(&mut self, pos: &ChunkPos) {
        self.chunk_storage.view_center = *pos;
    }

    pub fn get_block_state(&self, pos: &BlockPos) -> Option<BlockState> {
        self.chunk_storage.get_block_state(pos)
    }

    pub fn set_block_state(&mut self, pos: &BlockPos, state: BlockState) -> Option<BlockState> {
        self.chunk_storage.set_block_state(pos, state)
    }

    pub fn set_entity_pos(&mut self, entity_id: u32, new_pos: Vec3) -> Result<(), MoveEntityError> {
        let mut entity = self
            .entity_mut(entity_id)
            .ok_or(MoveEntityError::EntityDoesNotExist)?;

        let old_chunk = ChunkPos::from(entity.pos());
        let new_chunk = ChunkPos::from(&new_pos);
        // this is fine because we update the chunk below
        unsafe { entity.move_unchecked(new_pos) };
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
        let mut entity = self
            .entity_mut(entity_id)
            .ok_or(MoveEntityError::EntityDoesNotExist)?;
        let new_pos = entity.pos().with_delta(delta);

        let old_chunk = ChunkPos::from(entity.pos());
        let new_chunk = ChunkPos::from(&new_pos);
        // this is fine because we update the chunk below

        unsafe { entity.move_unchecked(new_pos) };
        if old_chunk != new_chunk {
            self.entity_storage
                .update_entity_chunk(entity_id, &old_chunk, &new_chunk);
        }
        Ok(())
    }

    pub fn add_entity(&mut self, id: u32, entity: EntityData) {
        self.entity_storage.insert(id, entity);
    }

    pub fn height(&self) -> u32 {
        self.chunk_storage.height
    }

    pub fn min_y(&self) -> i32 {
        self.chunk_storage.min_y
    }

    pub fn entity_data_by_id(&self, id: u32) -> Option<&EntityData> {
        self.entity_storage.get_by_id(id)
    }

    pub fn entity_data_mut_by_id(&mut self, id: u32) -> Option<&mut EntityData> {
        self.entity_storage.get_mut_by_id(id)
    }

    pub fn entity(&self, id: u32) -> Option<Entity<&World>> {
        let entity_data = self.entity_storage.get_by_id(id)?;
        let entity_ptr = unsafe { entity_data.as_const_ptr() };
        Some(Entity::new(self, id, entity_ptr))
    }

    pub fn entity_mut(&mut self, id: u32) -> Option<Entity<'_, &mut World>> {
        let entity_data = self.entity_storage.get_mut_by_id(id)?;
        let entity_ptr = unsafe { entity_data.as_ptr() };
        Some(Entity::new(self, id, entity_ptr))
    }

    pub fn entity_by_uuid(&self, uuid: &Uuid) -> Option<&EntityData> {
        self.entity_storage.get_by_uuid(uuid)
    }

    pub fn entity_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut EntityData> {
        self.entity_storage.get_mut_by_uuid(uuid)
    }

    /// Get an iterator over all entities.
    #[inline]
    pub fn entities(&self) -> std::collections::hash_map::Values<'_, u32, EntityData> {
        self.entity_storage.entities()
    }

    pub fn find_one_entity<F>(&self, mut f: F) -> Option<&EntityData>
    where
        F: FnMut(&EntityData) -> bool,
    {
        self.entity_storage.find_one_entity(|entity| f(entity))
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
