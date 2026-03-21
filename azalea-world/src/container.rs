use std::{
    collections::HashMap,
    fmt::{self, Display},
    sync::{Arc, Weak},
};

use azalea_core::registry_holder::RegistryHolder;
use azalea_registry::identifier::Identifier;
use bevy_ecs::{component::Component, resource::Resource};
use derive_more::{Deref, DerefMut};
use nohash_hasher::IntMap;
use parking_lot::RwLock;
use rustc_hash::FxHashMap;
use tracing::{debug, error};

use crate::{ChunkStorage, World};

/// A container of [`World`] instances.
///
/// Worlds are stored as a `Weak` pointer here, so if no clients are using a
/// world then it will be forgotten.
#[derive(Default, Resource)]
pub struct Worlds {
    // We just refer to the chunks here and don't include entities because there's not that many
    // cases where we'd want to get every entity in the world (just getting the entities in chunks
    // should work fine).

    // Entities are garbage collected (by manual reference counting in EntityUuidIndex) so we don't
    // need to worry about them here.

    // If it looks like we're relying on the server giving us unique world names, that's because we
    // are. An evil server could give us two worlds with the same name and then we'd have no way of
    // telling them apart. We hope most servers are nice and don't do that. Perhaps this should be
    // changed in the future to be configurable.
    pub map: FxHashMap<WorldName, Weak<RwLock<World>>>,
}

impl Worlds {
    pub fn new() -> Self {
        Worlds::default()
    }

    /// Get a world instance from the container.
    ///
    /// Returns `None` if none of the clients are in the requested world.
    pub fn get(&self, name: &WorldName) -> Option<Arc<RwLock<World>>> {
        self.map.get(name).and_then(|world| world.upgrade())
    }

    /// Add an empty world to the container (unless it already exists) and
    /// returns a strong reference to the world.
    #[must_use = "the world will be immediately forgotten if unused"]
    pub fn get_or_insert(
        &mut self,
        name: WorldName,
        height: u32,
        min_y: i32,
        default_registries: &RegistryHolder,
    ) -> Arc<RwLock<World>> {
        match self.map.get(&name).and_then(|world| world.upgrade()) {
            Some(existing_lock) => {
                let existing = existing_lock.read();
                if existing.chunks.height() != height {
                    error!(
                        "Shared world height mismatch: {} != {height}",
                        existing.chunks.height()
                    );
                }
                if existing.chunks.min_y() != min_y {
                    error!(
                        "Shared world min_y mismatch: {} != {min_y}",
                        existing.chunks.min_y()
                    );
                }
                existing_lock.clone()
            }
            _ => {
                let world = Arc::new(RwLock::new(World {
                    chunks: ChunkStorage::new(height, min_y),
                    entities_by_chunk: HashMap::new(),
                    entity_by_id: IntMap::default(),
                    registries: default_registries.clone(),
                }));
                debug!("Added new world {name:?}");
                self.map.insert(name, Arc::downgrade(&world));
                world
            }
        }
    }
}

/// The name of the [`World`] (aka dimension) that an entity is in.
///
/// If two entities share the same world name, then Azalea assumes that they're
/// in the same world.
#[derive(Clone, Component, Debug, Deref, DerefMut, Eq, Hash, PartialEq)]
#[doc(alias("dimension"))]
pub struct WorldName(pub Identifier);
impl WorldName {
    /// Create a new `WorldName` with the given name.
    pub fn new(name: &str) -> Self {
        Self(Identifier::new(name))
    }
}
impl Display for WorldName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl From<Identifier> for WorldName {
    fn from(ident: Identifier) -> Self {
        Self(ident)
    }
}
