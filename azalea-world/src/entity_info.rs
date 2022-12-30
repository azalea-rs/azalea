use crate::entity::{self, new_entity, EcsEntityId, EntityId, EntityUuid, Position};
use azalea_core::{ChunkPos, Vec3};
use bevy_ecs::{
    query::{QueryEntityError, QueryState, ReadOnlyWorldQuery, WorldQuery},
    world::{EntityMut, EntityRef},
};
use log::warn;
use nohash_hasher::{IntMap, IntSet};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{Arc, Weak},
};
use uuid::Uuid;

// How entity updates are processed (to avoid issues with shared worlds)
// - each bot contains a map of { entity id: updates received }
// - the shared world also contains a canonical "true" updates received for each
//   entity
// - when a client loads an entity, its "updates received" is set to the same as
//   the global "updates received"
// - when the shared world sees an entity for the first time, the "updates
//   received" is set to 1.
// - clients can force the shared "updates received" to 0 to make it so certain
//   entities (i.e. other bots in our swarm) don't get confused and updated by
//   other bots
// - when a client gets an update to an entity, we check if our "updates
//   received" is the same as the shared world's "updates received": if it is,
//   then process the update and increment the client's and shared world's
//   "updates received" if not, then we simply increment our local "updates
//   received" and do nothing else

/// Store a map of entities by ID. To get an iterator over all entities, use
/// `storage.shared.read().entities` [`WeakEntityStorage::entities`].
///
/// This is meant to be used with shared worlds.
///
/// You can access the shared storage with `world.shared.read()`.
#[derive(Debug, Default)]
pub struct PartialEntityInfos {
    pub shared: Arc<RwLock<WeakEntityInfos>>,

    /// The entity id of the player that owns this partial world. This will
    /// make [`PartialWorld::entity_mut`] pretend the entity doesn't exist so
    /// it doesn't get modified from outside sources.
    ///
    /// [`PartialWorld::entity_mut`]: crate::PartialWorld::entity_mut
    pub owner_entity_id: Option<EntityId>,
    /// A counter for each entity that tracks how many updates we've observed
    /// for it.
    ///
    /// This is used for shared worlds (i.e. swarms), to make sure we don't
    /// update entities twice on accident.
    pub updates_received: IntMap<EntityId, u32>,
    /// A set of all the entity ids in render distance.
    pub(crate) loaded_entity_ids: IntSet<EntityId>,
}

#[derive(Default)]
pub struct WeakEntityInfos {
    /// The number of `PartialWorld`s that have this entity loaded.
    /// (this is reference counting)
    pub(crate) entity_reference_count: IntMap<EntityId, usize>,
    /// An index of all the entity ids we know are in a chunk
    pub(crate) ids_by_chunk: HashMap<ChunkPos, IntSet<EntityId>>,
    /// An index of entity ids by their UUIDs
    pub(crate) id_by_uuid: HashMap<Uuid, EntityId>,

    /// The canonical number of updates we've gotten for every entity.
    pub updates_received: IntMap<EntityId, u32>,
}

impl PartialEntityInfos {
    pub fn new(shared: Arc<RwLock<WeakEntityInfos>>, owner_entity_id: Option<EntityId>) -> Self {
        if let Some(owner_entity_id) = owner_entity_id {
            shared.write().updates_received.insert(owner_entity_id, 0);
        }
        Self {
            shared,
            owner_entity_id,
            updates_received: IntMap::default(),
            loaded_entity_ids: IntSet::default(),
        }
    }

    /// Whether the entity with the given id is being loaded by this storage.
    /// If you want to check whether the entity is in the shared storage, use
    /// [`WeakEntityStorage::contains_id`].
    #[inline]
    pub fn limited_contains_id(&self, id: &EntityId) -> bool {
        self.loaded_entity_ids.contains(id)
    }

    /// Get an [`EntityId`] from this u32 entity ID if the entity is being
    /// loaded by this storage.
    ///
    /// Note that you can just create an `EntityId` directly if you want, and
    /// it'll work if the entity is being loaded by any storage.
    #[inline]
    pub fn limited_get_by_id(&self, id: u32) -> Option<EntityId> {
        if self.limited_contains_id(&EntityId(id)) {
            Some(EntityId(id))
        } else {
            None
        }
    }

    /// Returns whether we're allowed to update this entity (to prevent two
    /// clients in a shared world updating it twice), and acknowleges that
    /// we WILL update it if it's true. Don't call this unless you actually
    /// got an entity update that all other clients within render distance
    /// will get too.
    pub fn maybe_update(&mut self, id: EntityId) -> bool {
        let this_client_updates_received = self.updates_received.get(&id).copied();
        let shared_updates_received = self.shared.read().updates_received.get(&id).copied();

        let can_update = this_client_updates_received == shared_updates_received;
        if can_update {
            let new_updates_received = this_client_updates_received.unwrap_or(0) + 1;
            self.updates_received.insert(id, new_updates_received);
            self.shared
                .write()
                .updates_received
                .insert(id, new_updates_received);
            true
        } else {
            false
        }
    }

    /// Get a reference to an entity by its UUID, if it's being loaded by this
    /// storage.
    #[inline]
    pub fn limited_get_by_uuid(&self, uuid: &Uuid) -> Option<EntityId> {
        let shared = self.shared.read();
        let entity_id = shared.id_by_uuid.get(uuid)?;
        self.limited_get_by_id(entity_id.0)
    }

    /// Move an entity from its old chunk to a new chunk.
    #[inline]
    pub fn update_entity_chunk(
        &mut self,
        entity_id: EntityId,
        old_chunk: &ChunkPos,
        new_chunk: &ChunkPos,
    ) {
        self.shared
            .write()
            .update_entity_chunk(entity_id, old_chunk, new_chunk);
    }
}

/// This is useful so functions that return an IntSet of entity ids can return a
/// reference to nothing.
static EMPTY_ENTITY_ID_INTSET: Lazy<IntSet<EntityId>> = Lazy::new(|| IntSet::default());

impl WeakEntityInfos {
    pub fn new() -> Self {
        Self {
            entity_reference_count: IntMap::default(),
            ids_by_chunk: HashMap::default(),
            id_by_uuid: HashMap::default(),
            updates_received: IntMap::default(),
        }
    }

    /// Call this if a [`PartialEntityStorage`] just removed an entity.
    ///
    /// It'll
    /// decrease the reference count and remove the entity from the storage if
    /// there's no more references to it.
    ///
    /// Returns whether the entity was removed.
    pub fn remove_entity_if_unused(&mut self, id: EntityId, uuid: Uuid, chunk: ChunkPos) -> bool {
        if let Some(count) = self.entity_reference_count.get_mut(&id) {
            *count -= 1;
            if *count == 0 {
                self.entity_reference_count.remove(&id);
                return true;
            }
        } else {
            warn!("Tried to remove entity with id {id} but it was not found.");
            return false;
        }
        if self.ids_by_chunk.remove(&chunk).is_none() {
            warn!("Tried to remove entity with id {id} from chunk {chunk:?} but it was not found.");
        }
        if self.id_by_uuid.remove(&uuid).is_none() {
            warn!("Tried to remove entity with id {id} from uuid {uuid:?} but it was not found.");
        }
        if self.updates_received.remove(&id).is_none() {
            // if this happens it means we weren't tracking the updates_received for the
            // client (bad)
            warn!(
                "Tried to remove entity with id {id} from updates_received but it was not found."
            );
        }
        true
    }

    /// Remove a chunk from the storage if the entities in it have no strong
    /// references left.
    pub fn remove_chunk_if_unused(&mut self, chunk: &ChunkPos) {
        if let Some(entities) = self.ids_by_chunk.get(chunk) {
            if entities.is_empty() {
                self.ids_by_chunk.remove(chunk);
            }
        }
    }

    /// Whether the entity with the given id is in the shared storage.
    #[inline]
    pub fn contains_entity(&self, id: EntityId) -> bool {
        self.entity_reference_count.contains_key(&id)
    }

    /// Returns a set of entities in the given chunk.
    pub fn entities_in_chunk<F>(&self, chunk: &ChunkPos) -> &IntSet<EntityId> {
        self.ids_by_chunk
            .get(chunk)
            .unwrap_or(&EMPTY_ENTITY_ID_INTSET)
    }

    pub fn id_by_uuid(&self, uuid: &Uuid) -> Option<&EntityId> {
        self.id_by_uuid.get(uuid)
    }

    /// Move an entity from its old chunk to a new chunk.
    #[inline]
    pub fn update_entity_chunk(
        &mut self,
        entity_id: EntityId,
        old_chunk: &ChunkPos,
        new_chunk: &ChunkPos,
    ) {
        if let Some(entities) = self.ids_by_chunk.get_mut(old_chunk) {
            entities.remove(&entity_id);
        }
        self.ids_by_chunk
            .entry(*new_chunk)
            .or_default()
            .insert(entity_id);
    }

    /// Set an entity's position in the world when we already have references
    /// to the [`Position`] and [`Physics`] components.
    pub fn set_entity_pos(
        &mut self,
        entity_id: EntityId,
        new_pos: Vec3,
        pos: &mut entity::Position,
        physics: &mut entity::Physics,
    ) {
        let old_chunk = ChunkPos::from(&*pos);
        let new_chunk = ChunkPos::from(&new_pos);
        // this is fine because we update the chunk below
        unsafe { entity::move_unchecked(pos, physics, new_pos) };
        if old_chunk != new_chunk {
            self.update_entity_chunk(entity_id, &old_chunk, &new_chunk);
        }
    }
}

impl Debug for WeakEntityInfos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WeakEntityStorage")
            // .field("ecs", &self.ecs)
            .field("entity_reference_count", &self.entity_reference_count)
            .field("ids_by_chunk", &self.ids_by_chunk)
            .field("id_by_uuid", &self.id_by_uuid)
            .field("updates_received", &self.updates_received)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::entity::metadata;

    use super::*;
    use azalea_core::Vec3;

    #[test]
    fn test_store_entity() {
        let mut storage = PartialEntityInfos::default();
        assert!(storage.limited_get_by_id(0).is_none());
        assert!(storage.shared.read().get_by_id(0).is_none());

        let uuid = Uuid::from_u128(100);
        storage.insert(
            0,
            EntityData::new(
                uuid,
                Vec3::default(),
                EntityMetadata::Player(metadata::Player::default()),
            ),
        );
        assert_eq!(storage.limited_get_by_id(0).unwrap().uuid, uuid);
        assert_eq!(storage.shared.read().get_by_id(0).unwrap().uuid, uuid);

        storage.remove_by_id(0);
        assert!(storage.limited_get_by_id(0).is_none());
        assert!(storage.shared.read().get_by_id(0).is_none());
    }
}
