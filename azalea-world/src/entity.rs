use std::collections::HashMap;

use azalea_core::ChunkPos;
use azalea_entity::Entity;
use nohash_hasher::IntMap;

#[derive(Debug)]
pub struct EntityStorage {
    by_id: IntMap<u32, Entity>,
    // TODO: this doesn't work yet (should be updated in the set_pos method in azalea-entity)
    by_chunk: HashMap<ChunkPos, u32>,
}

impl EntityStorage {
    pub fn new() -> Self {
        Self {
            by_id: IntMap::default(),
            by_chunk: HashMap::default(),
        }
    }

    /// Add an entity to the storage.
    #[inline]
    pub fn insert(&mut self, entity: Entity) {
        self.by_id.insert(entity.id, entity);
    }

    /// Remove an entity from the storage by its id.
    #[inline]
    pub fn remove_by_id(&mut self, id: u32) {
        self.by_id.remove(&id);
    }

    /// Get a reference to an entity by its id.
    #[inline]
    pub fn get_by_id(&self, id: u32) -> Option<&Entity> {
        self.by_id.get(&id)
    }

    /// Get a mutable reference to an entity by its id.
    #[inline]
    pub fn get_mut_by_id(&mut self, id: u32) -> Option<&mut Entity> {
        self.by_id.get_mut(&id)
    }
}
