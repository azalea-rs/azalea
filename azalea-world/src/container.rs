use crate::WeakWorld;
use azalea_core::ResourceLocation;
use log::error;
use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};

/// A container of [`WeakWorld`]s. Worlds are stored as a Weak pointer here, so
/// if no clients are using a world it will be forgotten.
#[derive(Default)]
pub struct WeakWorldContainer {
    pub worlds: HashMap<ResourceLocation, Weak<WeakWorld>>,
}

impl WeakWorldContainer {
    pub fn new() -> Self {
        WeakWorldContainer {
            worlds: HashMap::new(),
        }
    }

    /// Get a world from the container.
    pub fn get(&self, name: &ResourceLocation) -> Option<Arc<WeakWorld>> {
        self.worlds.get(name).and_then(|world| world.upgrade())
    }

    /// Add an empty world to the container (or not if it already exists) and
    /// returns a strong reference to the world.
    #[must_use = "the world will be immediately forgotten if unused"]
    pub fn insert(&mut self, name: ResourceLocation, height: u32, min_y: i32) -> Arc<WeakWorld> {
        if let Some(existing) = self.worlds.get(&name).and_then(|world| world.upgrade()) {
            if existing.height() != height {
                error!(
                    "Shared dimension height mismatch: {} != {}",
                    existing.height(),
                    height,
                );
            }
            if existing.min_y() != min_y {
                error!(
                    "Shared world min_y mismatch: {} != {}",
                    existing.min_y(),
                    min_y,
                );
            }
            existing
        } else {
            let world = Arc::new(WeakWorld::new(height, min_y));
            self.worlds.insert(name, Arc::downgrade(&world));
            world
        }
    }
}
