use azalea_core::ChunkPos;
use azalea_entity::Entity;
use log::warn;
use nohash_hasher::{IntMap, IntSet};
use std::collections::HashMap;

#[derive(Debug)]
pub struct EntityStorage {
    by_id: IntMap<u32, Entity>,
    // TODO: this doesn't work yet (should be updated in the set_pos method in azalea-entity)
    by_chunk: HashMap<ChunkPos, IntSet<u32>>,
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
        self.by_chunk
            .entry(ChunkPos::from(entity.pos()))
            .or_default()
            .insert(entity.id);
        self.by_id.insert(entity.id, entity);
    }

    /// Remove an entity from the storage by its id.
    #[inline]
    pub fn remove_by_id(&mut self, id: u32) {
        if let Some(entity) = self.by_id.remove(&id) {
            let entity_chunk = ChunkPos::from(entity.pos());
            if self.by_chunk.remove(&entity_chunk).is_none() {
                warn!("Tried to remove entity with id {id} from chunk {entity_chunk:?} but it was not found.");
            }
        } else {
            warn!("Tried to remove entity with id {id} but it was not found.")
        }
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

    /// Clear all entities in a chunk.
    pub fn clear_chunk(&mut self, chunk: &ChunkPos) {
        if let Some(entities) = self.by_chunk.remove(chunk) {
            for entity_id in entities {
                self.by_id.remove(&entity_id);
            }
        }
    }

    /// Updates an entity from its old chunk.
    #[inline]
    pub fn update_entity_chunk(
        &mut self,
        entity_id: u32,
        old_chunk: &ChunkPos,
        new_chunk: &ChunkPos,
    ) {
        if let Some(entities) = self.by_chunk.get_mut(old_chunk) {
            entities.remove(&entity_id);
        }
        self.by_chunk
            .entry(*new_chunk)
            .or_default()
            .insert(entity_id);
    }
}

impl Default for EntityStorage {
    fn default() -> Self {
        Self::new()
    }
}
