use crate::{
    entity::{move_unchecked, EntityId, Physics, Position},
    Chunk, MoveEntityError, PartialChunkStorage, PartialEntityStorage, WeakChunkStorage,
    WeakEntityStorage,
};
use azalea_block::BlockState;
use azalea_buf::BufReadError;
use azalea_core::{BlockPos, ChunkPos, PositionDelta8, Vec3};
use bevy_ecs::query::{QueryState, WorldQuery};
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

    pub chunks: PartialChunkStorage,
    pub entities: PartialEntityStorage,
}

/// A world where the chunks are stored as weak pointers. This is used for
/// shared worlds.
#[derive(Default, Debug)]
pub struct WeakWorld {
    pub chunks: Arc<RwLock<WeakChunkStorage>>,
    pub entities: Arc<RwLock<WeakEntityStorage>>,
}

impl PartialWorld {
    pub fn new(
        chunk_radius: u32,
        shared: Arc<WeakWorld>,
        owner_entity_id: Option<EntityId>,
    ) -> Self {
        PartialWorld {
            shared: shared.clone(),
            chunks: PartialChunkStorage::new(chunk_radius, shared.chunks.clone()),
            entities: PartialEntityStorage::new(shared.entities.clone(), owner_entity_id),
        }
    }

    pub fn replace_with_packet_data(
        &mut self,
        pos: &ChunkPos,
        data: &mut Cursor<&[u8]>,
    ) -> Result<(), BufReadError> {
        self.chunks.replace_with_packet_data(pos, data)
    }

    pub fn get_chunk(&self, pos: &ChunkPos) -> Option<Arc<RwLock<Chunk>>> {
        self.chunks.get(pos)
    }

    pub fn set_chunk(&mut self, pos: &ChunkPos, chunk: Option<Chunk>) -> Result<(), BufReadError> {
        self.chunks
            .set(pos, chunk.map(|c| Arc::new(RwLock::new(c))));
        Ok(())
    }

    pub fn update_view_center(&mut self, pos: &ChunkPos) {
        self.chunks.view_center = *pos;
    }

    pub fn set_block_state(&mut self, pos: &BlockPos, state: BlockState) -> Option<BlockState> {
        self.chunks.set_block_state(pos, state)
    }

    /// Whether we're allowed to update the entity with the given ID.
    ///
    /// Only call this if you're actually updating the entity, otherwise it'll
    /// cause the update tracker to get out of sync.
    fn maybe_update_entity(&mut self, id: EntityId) -> bool {
        // no entity for you (we're processing this entity somewhere else)
        if Some(id) != self.entities.owner_entity_id && !self.entities.maybe_update(id) {
            false
        } else {
            true
        }
    }

    /// Makes sure we can modify this EntityId, and returns it if we can.
    ///
    /// Only call this if you're actually updating the entity, otherwise it'll
    /// cause the update tracker to get out of sync.
    pub fn entity_mut(&mut self, id: EntityId) -> Option<EntityId> {
        if self.maybe_update_entity(id) {
            Some(id)
        } else {
            None
        }
    }

    pub fn add_entity(&mut self, id: EntityId, bundle: impl bevy_ecs::prelude::Bundle) {
        self.entities.insert(id, bundle);
    }

    pub fn set_entity_pos(
        &mut self,
        entity_id: EntityId,
        new_pos: Vec3,
    ) -> Result<(), MoveEntityError> {
        if self.maybe_update_entity(entity_id) {
            self.shared.set_entity_pos(entity_id, new_pos)
        } else {
            Err(MoveEntityError::EntityDoesNotExist(Backtrace::capture()))
        }
    }

    pub fn move_entity_with_delta(
        &mut self,
        entity_id: EntityId,
        delta: &PositionDelta8,
    ) -> Result<(), MoveEntityError> {
        let mut query = self.shared.query_entities::<&Position>();
        let pos = **query
            .get(&self.shared.entities.read().ecs, entity_id.into())
            .map_err(|_| MoveEntityError::EntityDoesNotExist(Backtrace::capture()))?;

        let new_pos = pos.with_delta(delta);

        self.set_entity_pos(entity_id, new_pos)
    }
}

impl WeakWorld {
    pub fn new(height: u32, min_y: i32) -> Self {
        WeakWorld {
            chunks: Arc::new(RwLock::new(WeakChunkStorage::new(height, min_y))),
            entities: Arc::new(RwLock::new(WeakEntityStorage::new())),
        }
    }

    /// Read the total height of the world. You can add this to [`Self::min_y`]
    /// to get the highest possible y coordinate a block can be placed at.
    pub fn height(&self) -> u32 {
        self.chunks.read().height
    }

    /// Get the lowest possible y coordinate a block can be placed at.
    pub fn min_y(&self) -> i32 {
        self.chunks.read().min_y
    }

    pub fn id_by_uuid(&self, uuid: &Uuid) -> Option<EntityId> {
        self.entities.read().id_by_uuid(uuid).copied()
    }

    pub fn query_entities<Q: WorldQuery>(&self) -> QueryState<Q, ()> {
        self.entities.write().query_to_state::<Q>()
    }

    /// Set an entity's position in the world.
    ///
    /// Note that this will access the [`Position`] and [`Physics`] components,
    /// so if you already references to those components, you should use
    /// [`Self::set_entity_pos_from_refs`] instead.
    pub fn set_entity_pos(
        &self,
        entity_id: EntityId,
        new_pos: Vec3,
    ) -> Result<(), MoveEntityError> {
        let mut entity_storage = self.entities.write();
        let (pos, physics) =
            entity_storage.query_entity_mut::<(&mut Position, &mut Physics)>(entity_id);

        self.set_entity_pos_from_refs(entity_id, new_pos, pos.into_inner(), physics.into_inner())
    }

    /// Set an entity's position in the world when we already have references
    /// to the [`Position`] and [`Physics`] components.
    pub fn set_entity_pos_from_refs(
        &self,
        entity_id: EntityId,
        new_pos: Vec3,
        pos: &mut Position,
        physics: &mut Physics,
    ) -> Result<(), MoveEntityError> {
        let old_chunk = ChunkPos::from(&*pos);
        let new_chunk = ChunkPos::from(&new_pos);
        // this is fine because we update the chunk below
        unsafe { move_unchecked(pos, physics, new_pos) };
        if old_chunk != new_chunk {
            self.entities
                .write()
                .update_entity_chunk(entity_id, &old_chunk, &new_chunk);
        }
        Ok(())
    }

    pub fn get_block_state(&self, pos: &BlockPos) -> Option<BlockState> {
        self.chunks.read().get_block_state(pos)
    }

    pub fn get_chunk(&self, pos: &ChunkPos) -> Option<Arc<RwLock<Chunk>>> {
        self.chunks.read().get(pos)
    }
}

impl Debug for PartialWorld {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("World")
            .field("chunk_storage", &self.chunks)
            .field("entity_storage", &self.entities)
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
                chunks: chunk_storage.shared.clone(),
                entities: entity_storage.shared.clone(),
            }),
            chunks: chunk_storage,
            entities: entity_storage,
        }
    }
}
