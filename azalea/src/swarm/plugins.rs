use crate::{Swarm, SwarmEvent};
use async_trait::async_trait;
use nohash_hasher::NoHashHasher;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    hash::BuildHasherDefault,
};

type U64Hasher = BuildHasherDefault<NoHashHasher<u64>>;

// kind of based on https://docs.rs/http/latest/src/http/extensions.rs.html
/// A map of plugin ids to [`SwarmPlugin`] trait objects. The client stores
/// this so we can keep the state for our [`Swarm`] plugins.
///
/// If you're using azalea, you should generate this from the `swarm_plugins!` macro.
#[derive(Clone, Default)]
pub struct SwarmPlugins<S> {
    map: Option<HashMap<TypeId, Box<dyn SwarmPlugin<S>>, U64Hasher>>,
}

#[derive(Clone)]
pub struct SwarmPluginStates<S> {
    map: Option<HashMap<TypeId, Box<dyn SwarmPluginState<S>>, U64Hasher>>,
}

impl<S> SwarmPluginStates<S> {
    pub fn get<T: SwarmPluginState<S>>(&self) -> Option<&T> {
        self.map
            .as_ref()
            .and_then(|map| map.get(&TypeId::of::<T>()))
            .and_then(|boxed| (boxed.as_ref() as &dyn Any).downcast_ref::<T>())
    }
}

impl<S> SwarmPlugins<S>
where
    S: 'static,
{
    /// Create a new empty set of plugins.
    pub fn new() -> Self {
        Self { map: None }
    }

    /// Add a new plugin to this set.
    pub fn add<T: SwarmPlugin<S>>(&mut self, plugin: T) {
        if self.map.is_none() {
            self.map = Some(HashMap::with_hasher(BuildHasherDefault::default()));
        }
        self.map
            .as_mut()
            .unwrap()
            .insert(TypeId::of::<T>(), Box::new(plugin));
    }

    /// Build our plugin states from this set of plugins. Note that if you're
    /// using `azalea` you'll probably never need to use this as it's called
    /// for you.
    pub fn build(self) -> SwarmPluginStates<S> {
        if self.map.is_none() {
            return SwarmPluginStates { map: None };
        }
        let mut map = HashMap::with_hasher(BuildHasherDefault::default());
        for (id, plugin) in self.map.unwrap().into_iter() {
            map.insert(id, plugin.build());
        }
        SwarmPluginStates { map: Some(map) }
    }
}

impl<S> IntoIterator for SwarmPluginStates<S> {
    type Item = Box<dyn SwarmPluginState<S>>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    /// Iterate over the plugin states.
    fn into_iter(self) -> Self::IntoIter {
        self.map
            .map(|map| map.into_values().collect::<Vec<_>>())
            .unwrap_or_default()
            .into_iter()
    }
}

/// A `SwarmPluginState` keeps the current state of a plugin for a client. All
/// the fields must be atomic. Unique `SwarmPluginState`s are built from
/// [`SwarmPlugin`]s.
#[async_trait]
pub trait SwarmPluginState<S>: Send + Sync + SwarmPluginStateClone<S> + Any + 'static {
    async fn handle(self: Box<Self>, event: SwarmEvent, swarm: Swarm<S>);
}

/// Swarm plugins can keep their own personal state ([`SwarmPluginState`]),
/// listen to [`SwarmEvent`]s, and add new functions to [`Swarm`].
pub trait SwarmPlugin<S>: Send + Sync + SwarmPluginClone<S> + Any + 'static {
    fn build(&self) -> Box<dyn SwarmPluginState<S>>;
}

/// An internal trait that allows SwarmPluginState to be cloned.
#[doc(hidden)]
pub trait SwarmPluginStateClone<S> {
    fn clone_box(&self) -> Box<dyn SwarmPluginState<S>>;
}
impl<T, S> SwarmPluginStateClone<S> for T
where
    T: 'static + SwarmPluginState<S> + Clone,
{
    fn clone_box(&self) -> Box<dyn SwarmPluginState<S>> {
        Box::new(self.clone())
    }
}
impl<S> Clone for Box<dyn SwarmPluginState<S>> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// An internal trait that allows SwarmPlugin to be cloned.
#[doc(hidden)]
pub trait SwarmPluginClone<S> {
    fn clone_box(&self) -> Box<dyn SwarmPlugin<S>>;
}
impl<T, S> SwarmPluginClone<S> for T
where
    T: 'static + SwarmPlugin<S> + Clone,
{
    fn clone_box(&self) -> Box<dyn SwarmPlugin<S>> {
        Box::new(self.clone())
    }
}
impl<S> Clone for Box<dyn SwarmPlugin<S>> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
