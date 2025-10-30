use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};

use azalea_core::{registry_holder::RegistryHolder, resource_location::ResourceLocation};
use bevy_ecs::{component::Component, resource::Resource};
use derive_more::{Deref, DerefMut};
use nohash_hasher::IntMap;
use parking_lot::RwLock;
use rustc_hash::FxHashMap;
use tracing::{debug, error};

use crate::{ChunkStorage, Instance};

/// A container of [`Instance`]s (aka worlds).
///
/// Instances are stored as a Weak pointer here, so if no clients are using an
/// instance it will be forgotten.
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

    /// Add an empty world to the container (unless it already exists) and
    /// returns a strong reference to the world.
    #[must_use = "the world will be immediately forgotten if unused"]
    pub fn get_or_insert(
        &mut self,
        name: ResourceLocation,
        height: u32,
        min_y: i32,
        default_registries: &RegistryHolder,
    ) -> Arc<RwLock<Instance>> {
        match self.instances.get(&name).and_then(|world| world.upgrade()) {
            Some(existing_lock) => {
                let existing = existing_lock.read();
                if existing.chunks.height != height {
                    error!(
                        "Shared world height mismatch: {} != {height}",
                        existing.chunks.height
                    );
                }
                if existing.chunks.min_y != min_y {
                    error!(
                        "Shared world min_y mismatch: {} != {min_y}",
                        existing.chunks.min_y
                    );
                }
                existing_lock.clone()
            }
            _ => {
                let world = Arc::new(RwLock::new(Instance {
                    chunks: ChunkStorage::new(height, min_y),
                    entities_by_chunk: HashMap::new(),
                    entity_by_id: IntMap::default(),
                    registries: default_registries.clone(),
                }));
                debug!("Added new instance {name}");
                self.instances.insert(name, Arc::downgrade(&world));
                world
            }
        }
    }
}

/// The name of the [`Instance`] (aka world/dimension) that the entity is in.
///
/// If two entities share the same instance name, we assume they're in the
/// same instance.
#[derive(Component, Clone, Debug, PartialEq, Deref, DerefMut)]
#[doc(alias("worldname", "world name"))]
pub struct InstanceName(pub ResourceLocation);
