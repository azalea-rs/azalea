use crate::{
    entity::{Entity, EntityData},
    Chunk, MoveEntityError, PartialChunkStorage, PartialEntityStorage, WeakChunkStorage,
    WeakEntityStorage,
};
use azalea_block::BlockState;
use azalea_buf::BufReadError;
use azalea_core::{BlockPos, ChunkPos, PositionDelta8, Vec3};
use parking_lot::RwLock;
use std::{backtrace::Backtrace, fmt::Debug};
use std::{fmt::Formatter, io::Cursor, sync::Arc};
use uuid::Uuid;

/// PartialWorlds are usually owned by clients, and hold strong references to
/// chunks and entities in [`WeakWorld`]s.
///
/// Basically, they hold the chunks and entities that are within render
/// distance but can still access chunks and entities owned by other
/// `PartialWorld`s that have the same `WeakWorld`.
///
/// This is primarily useful for having multiple clients in the same world.
pub struct PartialWorld {
    // we just need to keep a strong reference to `shared` so it doesn't get
    // dropped, we don't need to do anything with it
    pub shared: Arc<WeakWorld>,

    pub chunk_storage: PartialChunkStorage,
    pub entity_storage: PartialEntityStorage,
}

/// A world where the chunks are stored as weak pointers. This is used for
/// shared worlds.
#[derive(Default, Debug)]
pub struct WeakWorld {
    pub chunk_storage: Arc<RwLock<WeakChunkStorage>>,
    pub entity_storage: Arc<RwLock<WeakEntityStorage>>,
}

impl PartialWorld {
    pub fn new(chunk_radius: u32, shared: Arc<WeakWorld>, owner_entity_id: Option<u32>) -> Self {
        PartialWorld {
            shared: shared.clone(),
            chunk_storage: PartialChunkStorage::new(chunk_radius, shared.chunk_storage.clone()),
            entity_storage: PartialEntityStorage::new(
                shared.entity_storage.clone(),
                owner_entity_id,
            ),
        }
    }

    pub fn replace_with_packet_data(
        &mut self,
        pos: &ChunkPos,
        data: &mut Cursor<&[u8]>,
    ) -> Result<(), BufReadError> {
        self.chunk_storage.replace_with_packet_data(pos, data)
    }

    pub fn get_chunk(&self, pos: &ChunkPos) -> Option<Arc<RwLock<Chunk>>> {
        self.chunk_storage.get(pos)
    }

    pub fn set_chunk(&mut self, pos: &ChunkPos, chunk: Option<Chunk>) -> Result<(), BufReadError> {
        self.chunk_storage
            .set(pos, chunk.map(|c| Arc::new(RwLock::new(c))));
        Ok(())
    }

    pub fn update_view_center(&mut self, pos: &ChunkPos) {
        self.chunk_storage.view_center = *pos;
    }

    pub fn set_block_state(&mut self, pos: &BlockPos, state: BlockState) -> Option<BlockState> {
        self.chunk_storage.set_block_state(pos, state)
    }

    /// Returns a mutable reference to the entity with the given ID.
    pub fn entity_mut(&mut self, id: u32) -> Option<Entity<'_, &WeakWorld>> {
        // no entity for you (we're processing this entity somewhere else)
        if Some(id) != self.entity_storage.owner_entity_id && !self.entity_storage.maybe_update(id)
        {
            return None;
        }

        self.shared.entity(id)
    }

    pub fn add_entity(&mut self, id: u32, entity: EntityData) {
        self.entity_storage.insert(id, entity);
    }

    pub fn set_entity_pos(&mut self, entity_id: u32, new_pos: Vec3) -> Result<(), MoveEntityError> {
        let mut entity = self
            .entity_mut(entity_id)
            .ok_or_else(|| MoveEntityError::EntityDoesNotExist(Backtrace::capture()))?;
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
            .ok_or_else(|| MoveEntityError::EntityDoesNotExist(Backtrace::capture()))?;
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
}

impl WeakWorld {
    pub fn new(height: u32, min_y: i32) -> Self {
        WeakWorld {
            chunk_storage: Arc::new(RwLock::new(WeakChunkStorage::new(height, min_y))),
            entity_storage: Arc::new(RwLock::new(WeakEntityStorage::new())),
        }
    }

    /// Read the total height of the world. You can add this to [`Self::min_y`]
    /// to get the highest possible y coordinate a block can be placed at.
    pub fn height(&self) -> u32 {
        self.chunk_storage.read().height
    }

    /// Get the lowest possible y coordinate a block can be placed at.
    pub fn min_y(&self) -> i32 {
        self.chunk_storage.read().min_y
    }

    pub fn entity_data_by_id(&self, id: u32) -> Option<Arc<EntityData>> {
        self.entity_storage.read().get_by_id(id)
    }

    /// Returns a entity with the given ID.
    ///
    /// The returned Entity can technically be mutated, but you should avoid
    /// doing any relative mutations.
    pub fn entity(&self, id: u32) -> Option<Entity<&WeakWorld>> {
        let entity_data = self.entity_storage.read().get_by_id(id)?;
        let entity_ptr = unsafe { entity_data.as_ptr() };
        Some(Entity::new(self, id, entity_ptr))
    }

    pub fn entity_by_uuid(&self, uuid: &Uuid) -> Option<Arc<EntityData>> {
        self.entity_storage.read().get_by_uuid(uuid)
    }

    pub fn entity_by<F>(&self, mut f: F) -> Option<Arc<EntityData>>
    where
        F: FnMut(&EntityData) -> bool,
    {
        self.entity_storage.read().entity_by(|e| f(e))
    }

    pub fn entities_by<F>(&self, mut f: F) -> Vec<Arc<EntityData>>
    where
        F: FnMut(&EntityData) -> bool,
    {
        self.entity_storage.read().entities_by(|e| f(e))
    }

    pub fn set_entity_pos(&self, entity_id: u32, new_pos: Vec3) -> Result<(), MoveEntityError> {
        let mut entity = self
            .entity(entity_id)
            .ok_or_else(|| MoveEntityError::EntityDoesNotExist(Backtrace::capture()))?;
        let old_chunk = ChunkPos::from(entity.pos());
        let new_chunk = ChunkPos::from(&new_pos);
        // this is fine because we update the chunk below
        unsafe { entity.move_unchecked(new_pos) };
        if old_chunk != new_chunk {
            self.entity_storage
                .write()
                .update_entity_chunk(entity_id, &old_chunk, &new_chunk);
        }
        Ok(())
    }

    pub fn get_block_state(&self, pos: &BlockPos) -> Option<BlockState> {
        self.chunk_storage.read().get_block_state(pos)
    }

    pub fn get_chunk(&self, pos: &ChunkPos) -> Option<Arc<RwLock<Chunk>>> {
        self.chunk_storage.read().get(pos)
    }
}

impl Debug for PartialWorld {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("World")
            .field("chunk_storage", &self.chunk_storage)
            .field("entity_storage", &self.entity_storage)
            .field("shared", &self.shared)
            .finish()
    }
}

impl Default for PartialWorld {
    fn default() -> Self {
        let chunk_storage = PartialChunkStorage::default();
        let entity_storage = PartialEntityStorage::default();
        Self {
            shared: Arc::new(WeakWorld {
                chunk_storage: chunk_storage.shared.clone(),
                entity_storage: entity_storage.shared.clone(),
            }),
            chunk_storage,
            entity_storage,
        }
    }
}
