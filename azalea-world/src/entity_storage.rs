use crate::entity::{EntityData, EntityId};
use azalea_core::ChunkPos;
use log::warn;
use nohash_hasher::{IntMap, IntSet};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug)]
pub struct EntityStorage {
    data_by_id: IntMap<EntityId, EntityData>,
    id_by_chunk: HashMap<ChunkPos, IntSet<EntityId>>,
    id_by_uuid: HashMap<Uuid, EntityId>,
}

impl EntityStorage {
    pub fn new() -> Self {
        Self {
            data_by_id: IntMap::default(),
            id_by_chunk: HashMap::default(),
            id_by_uuid: HashMap::default(),
        }
    }

    /// Add an entity to the storage.
    #[inline]
    pub fn insert(&mut self, id: EntityId, entity: EntityData) {
        self.id_by_chunk
            .entry(ChunkPos::from(entity.pos()))
            .or_default()
            .insert(id);
        self.id_by_uuid.insert(entity.uuid, id);
        self.data_by_id.insert(id, entity);
    }

    /// Remove an entity from the storage by its id.
    #[inline]
    pub fn remove_by_id(&mut self, id: EntityId) {
        if let Some(entity) = self.data_by_id.remove(&id) {
            let entity_chunk = ChunkPos::from(entity.pos());
            let entity_uuid = entity.uuid;
            if self.id_by_chunk.remove(&entity_chunk).is_none() {
                warn!("Tried to remove entity with id {id} from chunk {entity_chunk:?} but it was not found.");
            }
            if self.id_by_uuid.remove(&entity_uuid).is_none() {
                warn!("Tried to remove entity with id {id} from uuid {entity_uuid:?} but it was not found.");
            }
        } else {
            warn!("Tried to remove entity with id {id} but it was not found.")
        }
    }

    /// Check if there is an entity that exists with the given id.
    #[inline]
    pub fn contains_id(&self, id: &EntityId) -> bool {
        self.data_by_id.contains_key(id)
    }

    /// Get a reference to an entity by its id.
    #[inline]
    pub fn get_by_id(&self, id: EntityId) -> Option<&EntityData> {
        self.data_by_id.get(&id)
    }

    /// Get a mutable reference to an entity by its id.
    #[inline]
    pub fn get_mut_by_id<'d>(&'d mut self, id: EntityId) -> Option<&'d mut EntityData> {
        self.data_by_id.get_mut(&id)
    }

    /// Get a reference to an entity by its uuid.
    #[inline]
    pub fn get_by_uuid(&self, uuid: &Uuid) -> Option<&EntityData> {
        self.id_by_uuid
            .get(uuid)
            .and_then(|id| self.data_by_id.get(id))
    }

    /// Get a mutable reference to an entity by its uuid.
    #[inline]
    pub fn get_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut EntityData> {
        self.id_by_uuid
            .get(uuid)
            .and_then(|id| self.data_by_id.get_mut(id))
    }

    /// Clear all entities in a chunk.
    pub fn clear_chunk(&mut self, chunk: &ChunkPos) {
        if let Some(entities) = self.id_by_chunk.remove(chunk) {
            for entity_id in entities {
                if let Some(entity) = self.data_by_id.remove(&entity_id) {
                    self.id_by_uuid.remove(&entity.uuid);
                } else {
                    warn!("While clearing chunk {chunk:?}, found an entity that isn't in by_id {entity_id}.");
                }
            }
        }
    }

    /// Updates an entity from its old chunk.
    #[inline]
    pub fn update_entity_chunk(
        &mut self,
        entity_id: EntityId,
        old_chunk: &ChunkPos,
        new_chunk: &ChunkPos,
    ) {
        if let Some(entities) = self.id_by_chunk.get_mut(old_chunk) {
            entities.remove(&entity_id);
        }
        self.id_by_chunk
            .entry(*new_chunk)
            .or_default()
            .insert(entity_id);
    }

    /// Get an iterator over all entities.
    #[inline]
    pub fn entities(&self) -> std::collections::hash_map::Values<'_, EntityId, EntityData> {
        self.data_by_id.values()
    }

    pub fn find_one_entity<F>(&self, mut f: F) -> Option<&EntityData>
    where
        F: FnMut(&EntityData) -> bool,
    {
        self.entities().find(|&entity| f(entity))
    }

    pub fn find_one_entity_in_chunk<F>(&self, chunk: &ChunkPos, mut f: F) -> Option<&EntityData>
    where
        F: FnMut(&EntityData) -> bool,
    {
        if let Some(entities) = self.id_by_chunk.get(chunk) {
            for entity_id in entities {
                if let Some(entity) = self.data_by_id.get(entity_id) {
                    if f(entity) {
                        return Some(entity);
                    }
                }
            }
        }
        None
    }
}

impl Default for EntityStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azalea_core::Vec3;

    #[test]
    fn test_store_entity() {
        let mut storage = EntityStorage::new();
        assert!(storage.get_by_id(EntityId(0)).is_none());

        let uuid = Uuid::from_u128(100);
        storage.insert(EntityId(0), EntityData::new(uuid, Vec3::default()));
        assert_eq!(storage.get_by_id(EntityId(0)).unwrap().uuid, uuid);

        storage.remove_by_id(EntityId(0));
        assert!(storage.get_by_id(EntityId(0)).is_none());
    }
}
