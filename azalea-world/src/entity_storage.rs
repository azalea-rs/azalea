use crate::entity::EntityData;
use azalea_core::ChunkPos;
use log::warn;
use nohash_hasher::{IntMap, IntSet};
use parking_lot::RwLock;
use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};
use uuid::Uuid;

// How entity updates are processed (to avoid issues with shared worlds)
// - each bot contains a map of { entity id: updates received }
// - the shared world also contains a canonical "true" updates received for each entity
// - when a client loads an entity, its "updates received" is set to the same as the global "updates received"
// - when the shared world sees an entity for the first time, the "updates received" is set to 1.
// - clients can force the shared "updates received" to 0 to make it so certain entities (i.e. other bots in our swarm) don't get confused and updated by other bots
// - when a client gets an update to an entity, we check if our "updates received" is the same as the shared world's "updates received":
//      if it is, then process the update and increment the client's and shared world's "updates received"
//      if not, then we simply increment our local "updates received" and do nothing else

/// Store a map of entities by ID. To get an iterator over all entities, use
/// `storage.shared.read().entities` [`WeakEntityStorage::entities`].
///
/// This is meant to be used with shared worlds.
#[derive(Debug, Default)]
pub struct PartialEntityStorage {
    pub shared: Arc<RwLock<WeakEntityStorage>>,

    /// The entity id of the player that owns this struct.
    pub owner_entity_id: u32,
    pub updates_received: IntMap<u32, u32>,
    /// Strong references to the entities we have loaded.
    data_by_id: IntMap<u32, Arc<EntityData>>,
}

/// Weakly store entities in a world. If the entities aren't being referenced
/// by anything else (like an [`PartialEntityStorage`]), they'll be forgotten.
#[derive(Debug, Default)]
pub struct WeakEntityStorage {
    data_by_id: IntMap<u32, Weak<EntityData>>,
    /// An index of all the entity ids we know are in a chunk
    ids_by_chunk: HashMap<ChunkPos, IntSet<u32>>,
    /// An index of entity ids by their UUIDs
    id_by_uuid: HashMap<Uuid, u32>,

    pub updates_received: IntMap<u32, u32>,
}

impl PartialEntityStorage {
    pub fn new(shared: Arc<RwLock<WeakEntityStorage>>, owner_entity_id: u32) -> Self {
        shared.write().updates_received.insert(owner_entity_id, 0);
        Self {
            shared,
            owner_entity_id,
            updates_received: IntMap::default(),
            data_by_id: IntMap::default(),
        }
    }

    /// Add an entity to the storage.
    #[inline]
    pub fn insert(&mut self, id: u32, entity: EntityData) {
        // if the entity is already in the shared world, we don't need to do anything
        if self.shared.read().data_by_id.contains_key(&id) {
            return;
        }

        // add the entity to the "indexes"
        let mut shared = self.shared.write();
        shared
            .ids_by_chunk
            .entry(ChunkPos::from(entity.pos()))
            .or_default()
            .insert(id);
        shared.id_by_uuid.insert(entity.uuid, id);

        // now store the actual entity data
        let entity = Arc::new(entity);
        shared.data_by_id.insert(id, Arc::downgrade(&entity));
        self.data_by_id.insert(id, entity);
        // set our updates_received to the shared updates_received, unless it's
        // not there in which case set both to 1
        if let Some(&shared_updates_received) = shared.updates_received.get(&id) {
            // 0 means we're never tracking updates for this entity
            if shared_updates_received != 0 || id == self.owner_entity_id {
                self.updates_received.insert(id, 1);
            }
        } else {
            shared.updates_received.insert(id, 1);
            self.updates_received.insert(id, 1);
        }
    }

    /// Remove an entity from this storage by its id. It will only be removed
    /// from the shared storage if there are no other references to it.
    #[inline]
    pub fn remove_by_id(&mut self, id: u32) {
        if let Some(entity) = self.data_by_id.remove(&id) {
            let chunk = ChunkPos::from(entity.pos());
            let uuid = entity.uuid;
            self.updates_received.remove(&id);
            drop(entity);
            // maybe remove it from the storage
            self.shared.write().remove_entity_if_unused(id, uuid, chunk);
        } else {
            warn!("Tried to remove entity with id {id} but it was not found.")
        }
    }

    /// Whether the entity with the given id is being loaded by this storage.
    /// If you want to check whether the entity is in the shared storage, use
    /// [`WeakEntityStorage::contains_id`].
    #[inline]
    pub fn limited_contains_id(&self, id: &u32) -> bool {
        self.data_by_id.contains_key(id)
    }

    /// Whether the entity with the given id is in the shared storage (i.e.
    /// it's possible we don't see the entity but something else in the shared
    /// storage does). To check whether the entity is being loaded by this
    /// storage, use [`PartialEntityStorage::limited_contains_id`].
    #[inline]
    pub fn contains_id(&self, id: &u32) -> bool {
        self.shared.read().data_by_id.contains_key(id)
    }

    /// Get a reference to an entity by its id, if it's being loaded by this storage.
    #[inline]
    pub fn limited_get_by_id(&self, id: u32) -> Option<&Arc<EntityData>> {
        self.data_by_id.get(&id)
    }

    /// Get a mutable reference to an entity by its id, if it's being loaded by
    /// this storage.
    #[inline]
    pub fn limited_get_mut_by_id(&mut self, id: u32) -> Option<&mut Arc<EntityData>> {
        self.data_by_id.get_mut(&id)
    }

    /// Returns whether we're allowed to update this entity (to prevent two clients in
    /// a shared world updating it twice), and acknowleges that we WILL update
    /// it if it's true. Don't call this unless you actually got an entity
    /// update that all other clients within render distance will get too.
    pub fn maybe_update(&mut self, id: u32) -> bool {
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

    /// Get an entity in the shared storage by its id, if it exists.
    #[inline]
    pub fn get_by_id(&self, id: u32) -> Option<Arc<EntityData>> {
        self.shared
            .read()
            .data_by_id
            .get(&id)
            .and_then(|e| e.upgrade())
    }

    /// Get a reference to an entity by its UUID, if it's being loaded by this
    /// storage.
    #[inline]
    pub fn limited_get_by_uuid(&self, uuid: &Uuid) -> Option<&Arc<EntityData>> {
        self.shared
            .read()
            .id_by_uuid
            .get(uuid)
            .and_then(|id| self.data_by_id.get(id))
    }

    /// Get a mutable reference to an entity by its UUID, if it's being loaded
    /// by this storage.
    #[inline]
    pub fn limited_get_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut Arc<EntityData>> {
        self.shared
            .read()
            .id_by_uuid
            .get(uuid)
            .and_then(|id| self.data_by_id.get_mut(id))
    }

    /// Get an entity in the shared storage by its UUID, if it exists.
    #[inline]
    pub fn get_by_uuid(&self, uuid: &Uuid) -> Option<Arc<EntityData>> {
        self.shared.read().id_by_uuid.get(uuid).and_then(|id| {
            self.shared
                .read()
                .data_by_id
                .get(id)
                .and_then(|e| e.upgrade())
        })
    }

    /// Clear all entities in a chunk. This will not clear them from the
    /// shared storage, unless there are no other references to them.
    pub fn clear_chunk(&mut self, chunk: &ChunkPos) {
        if let Some(entities) = self.shared.read().ids_by_chunk.get(chunk) {
            for id in entities.iter() {
                if let Some(entity) = self.data_by_id.remove(id) {
                    let uuid = entity.uuid;
                    drop(entity);
                    // maybe remove it from the storage
                    self.shared
                        .write()
                        .remove_entity_if_unused(*id, uuid, *chunk);
                }
            }
            //     for entity_id in entities {
            //         self.remove_by_id(entity_id);
            //     }
        }
    }

    /// Move an entity from its old chunk to a new chunk.
    #[inline]
    pub fn update_entity_chunk(
        &mut self,
        entity_id: u32,
        old_chunk: &ChunkPos,
        new_chunk: &ChunkPos,
    ) {
        if let Some(entities) = self.shared.write().ids_by_chunk.get_mut(old_chunk) {
            entities.remove(&entity_id);
        }
        self.shared
            .write()
            .ids_by_chunk
            .entry(*new_chunk)
            .or_default()
            .insert(entity_id);
    }

    pub fn find_one_entity<F>(&self, mut f: F) -> Option<Arc<EntityData>>
    where
        F: FnMut(&Arc<EntityData>) -> bool,
    {
        for entity in self.shared.read().entities() {
            if let Some(entity) = entity.upgrade() {
                if f(&entity) {
                    return Some(entity);
                }
            }
        }
        None
    }

    pub fn find_one_entity_in_chunk<F>(&self, chunk: &ChunkPos, mut f: F) -> Option<Arc<EntityData>>
    where
        F: FnMut(&EntityData) -> bool,
    {
        let shared = self.shared.read();
        if let Some(entities) = shared.ids_by_chunk.get(chunk) {
            for entity_id in entities {
                if let Some(entity) = shared.data_by_id.get(entity_id).and_then(|e| e.upgrade()) {
                    if f(&entity) {
                        return Some(entity);
                    }
                }
            }
        }
        None
    }
}

impl WeakEntityStorage {
    pub fn new() -> Self {
        Self {
            data_by_id: IntMap::default(),
            ids_by_chunk: HashMap::default(),
            id_by_uuid: HashMap::default(),
            updates_received: IntMap::default(),
        }
    }

    /// Remove an entity from the storage if it has no strong references left.
    /// Returns whether the entity was removed.
    pub fn remove_entity_if_unused(&mut self, id: u32, uuid: Uuid, chunk: ChunkPos) -> bool {
        if self.data_by_id.get(&id).and_then(|e| e.upgrade()).is_some() {
            // if we could get the entity, that means there are still strong
            // references to it
            false
        } else {
            if self.ids_by_chunk.remove(&chunk).is_none() {
                warn!("Tried to remove entity with id {id} from chunk {chunk:?} but it was not found.");
            }
            if self.id_by_uuid.remove(&uuid).is_none() {
                warn!(
                    "Tried to remove entity with id {id} from uuid {uuid:?} but it was not found."
                );
            }
            if self.updates_received.remove(&id).is_none() {
                // if this happens it means we weren't tracking the updates_received for the client (bad)
                warn!(
                    "Tried to remove entity with id {id} from updates_received but it was not found."
                );
            }
            true
        }
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

    /// Get an iterator over all entities in the shared storage. The iterator
    /// is over `Weak<EntityData>`s, so you'll have to manually try to upgrade.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::sync::Arc;
    /// # use azalea_world::{PartialEntityStorage, entity::{EntityData, EntityMetadata, metadata}};
    /// # use azalea_core::Vec3;
    /// # use uuid::Uuid;
    /// #
    /// let mut storage = PartialEntityStorage::default();
    /// storage.insert(
    ///     0,
    ///     EntityData::new(
    ///         Uuid::nil(),
    ///         Vec3::default(),
    ///         EntityMetadata::Player(metadata::Player::default()),
    ///     ),
    /// );
    /// for entity in storage.shared.read().entities() {
    ///     if let Some(entity) = entity.upgrade() {
    ///         println!("Entity: {:?}", entity);
    ///     }
    /// }
    /// ```
    pub fn entities(&self) -> std::collections::hash_map::Values<'_, u32, Weak<EntityData>> {
        self.data_by_id.values()
    }

    /// Whether the entity with the given id is in the shared storage.
    #[inline]
    pub fn contains_id(&self, id: &u32) -> bool {
        self.data_by_id.contains_key(id)
    }
}

#[cfg(test)]
mod tests {
    use crate::entity::{metadata, EntityMetadata};

    use super::*;
    use azalea_core::Vec3;

    #[test]
    fn test_store_entity() {
        let mut storage = PartialEntityStorage::default();
        assert!(storage.get_by_id(0).is_none());

        let uuid = Uuid::from_u128(100);
        storage.insert(
            0,
            EntityData::new(
                uuid,
                Vec3::default(),
                EntityMetadata::Player(metadata::Player::default()),
            ),
        );
        assert_eq!(storage.get_by_id(0).unwrap().uuid, uuid);

        storage.remove_by_id(0);
        assert!(storage.get_by_id(0).is_none());
    }
}
