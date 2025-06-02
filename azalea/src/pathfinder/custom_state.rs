use std::{
    any::{Any, TypeId},
    sync::Arc,
};

use bevy_ecs::component::Component;
use parking_lot::RwLock;
use rustc_hash::FxHashMap;

/// The component that holds the custom pathfinder state for one of our bots.
///
/// See [`CustomPathfinderStateRef`] for more information about the inner type.
///
/// Azalea won't automatically insert this component, so if you're trying to use
/// it then you should also have logic to insert the component if it's not
/// present.
///
/// Be aware that a read lock is held on the `RwLock` while a path is being
/// calculated, which may take up to several seconds. For this reason, it may be
/// favorable to use [`RwLock::try_write`] instead of [`RwLock::write`] when
/// updating it to avoid blocking the current thread.
#[derive(Clone, Component, Default)]
pub struct CustomPathfinderState(pub Arc<RwLock<CustomPathfinderStateRef>>);

/// Arbitrary state that's passed to the pathfinder, intended to be used for
/// custom moves that need to access things that are usually inaccessible.
///
/// This is included in [`PathfinderCtx`].
///
/// [`PathfinderCtx`]: crate::pathfinder::PathfinderCtx
#[derive(Debug, Default)]
pub struct CustomPathfinderStateRef {
    map: FxHashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl CustomPathfinderStateRef {
    pub fn insert<T: 'static + Send + Sync>(&mut self, t: T) {
        self.map.insert(TypeId::of::<T>(), Box::new(t));
    }

    pub fn get<T: 'static + Send + Sync>(&self) -> Option<&T> {
        self.map
            .get(&TypeId::of::<T>())
            .map(|value| value.downcast_ref().unwrap())
    }
}
