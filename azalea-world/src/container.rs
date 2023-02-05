use azalea_core::ResourceLocation;
use azalea_ecs::system::Resource;
use log::error;
use nohash_hasher::IntMap;
use parking_lot::RwLock;
use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};

use crate::{ChunkStorage, World};

/// A container of [`World`]s. Worlds are stored as a Weak pointer here, so
/// if no clients are using a world it will be forgotten.
#[derive(Default, Resource)]
pub struct WorldContainer {
    // We just refer to the chunks here and don't include entities because there's not that many
    // cases where we'd want to get every entity in the world (just getting the entities in chunks
    // should work fine).

    // Entities are garbage collected (by manual reference counting in EntityInfos) so we don't
    // need to worry about them here.

    // If it looks like we're relying on the server giving us unique world names, that's because we
    // are. An evil server could give us two worlds with the same name and then we'd have no way of
    // telling them apart. We hope most servers are nice and don't do that though. It's only an
    // issue when there's multiple clients with the same WorldContainer in different worlds
    // anyways.
    pub worlds: HashMap<ResourceLocation, Weak<RwLock<World>>>,
}

impl WorldContainer {
    pub fn new() -> Self {
        WorldContainer {
            worlds: HashMap::new(),
        }
    }

    /// Get a world from the container.
    pub fn get(&self, name: &ResourceLocation) -> Option<Arc<RwLock<World>>> {
        self.worlds.get(name).and_then(|world| world.upgrade())
    }

    /// Add an empty world to the container (or not if it already exists) and
    /// returns a strong reference to the world.
    #[must_use = "the world will be immediately forgotten if unused"]
    pub fn insert(
        &mut self,
        name: ResourceLocation,
        height: u32,
        min_y: i32,
    ) -> Arc<RwLock<World>> {
        if let Some(existing_lock) = self.worlds.get(&name).and_then(|world| world.upgrade()) {
            let existing = existing_lock.read();
            if existing.chunks.height != height {
                error!(
                    "Shared dimension height mismatch: {} != {}",
                    existing.chunks.height, height,
                );
            }
            if existing.chunks.min_y != min_y {
                error!(
                    "Shared world min_y mismatch: {} != {}",
                    existing.chunks.min_y, min_y,
                );
            }
            existing_lock.clone()
        } else {
            let world = Arc::new(RwLock::new(World {
                chunks: ChunkStorage::new(height, min_y),
                entities_by_chunk: HashMap::new(),
                entity_by_id: IntMap::default(),
            }));
            self.worlds.insert(name, Arc::downgrade(&world));
            world
        }
    }
}
