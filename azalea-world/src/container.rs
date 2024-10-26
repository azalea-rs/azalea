use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};

use azalea_core::{registry_holder::RegistryHolder, resource_location::ResourceLocation};
use bevy_ecs::{component::Component, system::Resource};
use derive_more::{Deref, DerefMut};
use nohash_hasher::IntMap;
use parking_lot::RwLock;
use rustc_hash::FxHashMap;
use tracing::error;

use crate::{ChunkStorage, Instance};

/// A container of [`Instance`]s (aka worlds). Instances are stored as a Weak
/// pointer here, so if no clients are using an instance it will be forgotten.
#[derive(Default, Resource)]
pub struct InstanceContainer {
    // We just refer to the chunks here and don't include entities because there's not that many
    // cases where we'd want to get every entity in the world (just getting the entities in chunks
    // should work fine).

    // Entities are garbage collected (by manual reference counting in EntityUuidIndex) so we don't
    // need to worry about them here.

    // If it looks like we're relying on the server giving us unique world names, that's because we
    // are. An evil server could give us two worlds with the same name and then we'd have no way of
    // telling them apart. We hope most servers are nice and don't do that though. It's only an
    // issue when there's multiple clients with the same WorldContainer in different worlds
    // anyways.
    pub instances: FxHashMap<ResourceLocation, Weak<RwLock<Instance>>>,
}

impl InstanceContainer {
    pub fn new() -> Self {
        InstanceContainer::default()
    }

    /// Get a world from the container. Returns `None` if none of the clients
    /// are in this world.
    pub fn get(&self, name: &InstanceName) -> Option<Arc<RwLock<Instance>>> {
        self.instances.get(name).and_then(|world| world.upgrade())
    }

    /// Add an empty world to the container (or not if it already exists) and
    /// returns a strong reference to the world.
    #[must_use = "the world will be immediately forgotten if unused"]
    pub fn insert(
        &mut self,
        name: ResourceLocation,
        height: u32,
        min_y: i32,
    ) -> Arc<RwLock<Instance>> {
        if let Some(existing_lock) = self.instances.get(&name).and_then(|world| world.upgrade()) {
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
            let world = Arc::new(RwLock::new(Instance {
                chunks: ChunkStorage::new(height, min_y),
                entities_by_chunk: HashMap::new(),
                entity_by_id: IntMap::default(),
                registries: RegistryHolder::default(),
            }));
            self.instances.insert(name, Arc::downgrade(&world));
            world
        }
    }
}

/// The name of the [`Instance`](crate::Instance) (world) the entity is
/// in. If two entities share the same world name, we assume they're in the same
/// instance.
#[derive(Component, Clone, Debug, PartialEq, Deref, DerefMut)]
pub struct InstanceName(pub ResourceLocation);
