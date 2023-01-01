use crate::{
    entity::{self, Entity, MinecraftEntityId},
    Chunk, EntityInfos, MoveEntityError, PartialChunkStorage, PartialEntityInfos, WeakChunkStorage,
};
use azalea_block::BlockState;
use azalea_buf::BufReadError;
use azalea_core::{BlockPos, ChunkPos, PositionDelta8, Vec3};
use log::warn;
use parking_lot::{Mutex, RwLock};
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
    pub shared: Arc<World>,

    pub chunks: PartialChunkStorage,
    pub entity_infos: PartialEntityInfos,
}

impl PartialWorld {
    pub fn new(
        chunk_radius: u32,
        shared: Arc<World>,
        owner_entity: Option<Entity>,
        entity_infos: &mut EntityInfos,
    ) -> Self {
        PartialWorld {
            shared: shared.clone(),
            chunks: PartialChunkStorage::new(chunk_radius, shared.chunks.clone()),
            entity_infos: PartialEntityInfos::new(owner_entity, entity_infos),
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
    fn maybe_update_entity(&mut self, entity: Entity, entity_infos: &mut EntityInfos) -> bool {
        // no entity for you (we're processing this entity somewhere else)
        if Some(entity) != self.entity_infos.owner_entity
            && !self.entity_infos.maybe_update(id, entity_infos)
        {
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

    pub fn set_entity_pos(
        &mut self,
        entity_id: EntityId,
        new_pos: Vec3,
        pos: &mut entity::Position,
        physics: &mut entity::Physics,
    ) -> Result<(), MoveEntityError> {
        if self.maybe_update_entity(entity_id) {
            self.shared
                .entity_infos
                .write()
                .set_entity_pos(entity_id, new_pos, pos, physics);
            Ok(())
        } else {
            Err(MoveEntityError::EntityDoesNotExist(Backtrace::capture()))
        }
    }

    pub fn move_entity_with_delta(
        &mut self,
        entity_id: EntityId,
        delta: &PositionDelta8,
    ) -> Result<(), MoveEntityError> {
        let global_ecs_lock = self.shared.global_ecs.clone();
        let mut ecs = global_ecs_lock.lock();
        let mut query = ecs.query::<(&mut entity::Position, &mut entity::Physics)>();
        let (mut pos, mut physics) = query
            .get_mut(&mut ecs, entity_id.into())
            .map_err(|_| MoveEntityError::EntityDoesNotExist(Backtrace::capture()))?;

        let new_pos = pos.with_delta(delta);

        self.set_entity_pos(entity_id, new_pos, &mut pos, &mut physics)
    }

    /// Add an entity to the storage.
    #[inline]
    pub fn add_entity(&mut self, id: EntityId, bundle: impl bevy_ecs::bundle::Bundle) {
        // if the entity is already in the shared world, we don't need to do
        // anything
        if self.shared.contains_entity(id) {
            return;
        }

        let mut ecs = self.shared.global_ecs.lock();
        entity::new_entity(&mut ecs, id, bundle);

        let mut query = ecs.query::<(&entity::EntityUuid, &entity::Position)>();
        let (&uuid, &pos) = query.get(&mut ecs, id.into()).unwrap();

        let partial_entity_infos = &mut self.entity_infos;
        let mut shared_entity_infos = self.shared.entity_infos.write();

        // add the entity to the indexes
        shared_entity_infos
            .ids_by_chunk
            .entry(ChunkPos::from(&pos))
            .or_default()
            .insert(id);
        shared_entity_infos.id_by_uuid.insert(*uuid, id);
        partial_entity_infos.loaded_entity_ids.insert(id);
        // set our updates_received to the shared updates_received, unless it's
        // not there in which case set both to 1
        if let Some(&shared_updates_received) = shared_entity_infos.updates_received.get(&id) {
            // 0 means we're never tracking updates for this entity
            if shared_updates_received != 0 || Some(id) == partial_entity_infos.owner_entity_id {
                partial_entity_infos.updates_received.insert(id, 1);
            }
        } else {
            shared_entity_infos.updates_received.insert(id, 1);
            partial_entity_infos.updates_received.insert(id, 1);
        }
    }

    /// Remove an entity from this storage by its id. It will only be removed
    /// from the shared storage if there are no other references to it.
    #[inline]
    pub fn remove_entity_by_id(&mut self, id: EntityId) {
        let partial_entity_infos = &mut self.entity_infos;
        let mut shared_entity_infos = self.shared.entity_infos.write();

        if partial_entity_infos.loaded_entity_ids.remove(&id) {
            let mut ecs = self.shared.global_ecs.lock();

            let mut query = ecs.query::<(&entity::EntityUuid, &entity::Position)>();
            let (uuid, pos) = query.get(&mut ecs, id.into()).expect(
                "If the entity was being loaded by this storage, it must be in the shared
                    storage.",
            );
            let chunk = ChunkPos::from(pos);
            let uuid = **uuid;

            // TODO: is this line actually necessary? test if it doesn't immediately
            // panic/deadlock without this line
            drop(query);

            partial_entity_infos.updates_received.remove(&id);
            shared_entity_infos.remove_entity_if_unused(id, uuid, chunk);
        } else {
            warn!("Tried to remove entity with id {id} but it was not found.")
        }
    }

    /// Clear all entities in a chunk. This will not clear them from the
    /// shared storage, unless there are no other references to them.
    pub fn clear_entities_in_chunk(&mut self, chunk: &ChunkPos) {
        let partial_entity_infos = &mut self.entity_infos;
        let mut shared_entity_infos = self.shared.entity_infos.write();

        let mut ecs = self.shared.global_ecs.lock();

        let mut query = ecs.query::<&entity::EntityUuid>();
        if let Some(entities) = shared_entity_infos.ids_by_chunk.get(chunk).cloned() {
            for &id in &entities {
                if partial_entity_infos.loaded_entity_ids.remove(&id) {
                    let uuid = **query.get(&ecs, id.into()).unwrap();
                    // maybe remove it from the storage
                    shared_entity_infos.remove_entity_if_unused(id, uuid, *chunk);
                }
            }
        }
    }
}

// /// A world where the chunks are stored as weak pointers. This is used for
// /// shared worlds.
// #[derive(Default, Debug)]
// pub struct World {
//     pub chunks: Arc<RwLock<WeakChunkStorage>>,
// }

// impl World {
//     pub fn new(height: u32, min_y: i32) -> Self {
//         World {
//             chunks: Arc::new(RwLock::new(WeakChunkStorage::new(height,
// min_y))),         }
//     }

//     /// Read the total height of the world. You can add this to
// [`Self::min_y`]     /// to get the highest possible y coordinate a block can
// be placed at.     pub fn height(&self) -> u32 {
//         self.chunks.read().height
//     }

//     /// Get the lowest possible y coordinate a block can be placed at.
//     pub fn min_y(&self) -> i32 {
//         self.chunks.read().min_y
//     }

//     pub fn contains_entity(&self, id: EntityId) -> bool {
//         self.entity_infos.read().contains_entity(id)
//     }

//     pub fn id_by_uuid(&self, uuid: &Uuid) -> Option<EntityId> {
//         self.entity_infos.read().id_by_uuid(uuid).copied()
//     }

//     pub fn get_block_state(&self, pos: &BlockPos) -> Option<BlockState> {
//         self.chunks.read().get_block_state(pos)
//     }

//     pub fn get_chunk(&self, pos: &ChunkPos) -> Option<Arc<RwLock<Chunk>>> {
//         self.chunks.read().get(pos)
//     }

//     pub fn set_entity_pos(
//         &self,
//         entity_id: EntityId,
//         new_pos: Vec3,
//         pos: &mut entity::Position,
//         physics: &mut entity::Physics,
//     ) {
//         self.entity_infos
//             .write()
//             .set_entity_pos(entity_id, new_pos, pos, physics);
//     }
// }

impl Debug for PartialWorld {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("World")
            .field("chunk_storage", &self.chunks)
            .field("entity_storage", &self.entity_infos)
            .field("shared", &self.shared)
            .finish()
    }
}

impl Default for PartialWorld {
    /// Creates a completely self-contained `PartialWorld`. This is only for
    /// testing and shouldn't be used in actual code!
    fn default() -> Self {
        let chunk_storage = PartialChunkStorage::default();
        let entity_storage = PartialEntityInfos::default();
        Self {
            shared: Arc::new(World {
                chunks: chunk_storage.shared.clone(),
                entity_infos: entity_storage.shared.clone(),
                global_ecs: Arc::new(Mutex::new(bevy_ecs::world::World::default())),
            }),
            chunks: chunk_storage,
            entity_infos: entity_storage,
        }
    }
}
