use crate::{Client, Event};
use async_trait::async_trait;
use nohash_hasher::NoHashHasher;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    hash::BuildHasherDefault,
};

type U64Hasher = BuildHasherDefault<NoHashHasher<u64>>;

// kind of based on https://docs.rs/http/latest/src/http/extensions.rs.html
#[derive(Clone, Default)]
pub struct PluginStates {
    map: Option<HashMap<TypeId, Box<dyn PluginState>, U64Hasher>>,
}

/// A map of PluginState TypeIds to AnyPlugin objects. This can then be built
/// into a [`PluginStates`] object to get a fresh new state based on this
/// plugin.
///
/// If you're using the azalea crate, you should generate this from the
/// `plugins!` macro.
#[derive(Clone, Default)]
pub struct Plugins {
    map: Option<HashMap<TypeId, Box<dyn AnyPlugin>, U64Hasher>>,
}

impl PluginStates {
    pub fn get<T: PluginState>(&self) -> Option<&T> {
        self.map
            .as_ref()
            .and_then(|map| map.get(&TypeId::of::<T>()))
            .and_then(|boxed| (boxed.as_ref() as &dyn Any).downcast_ref::<T>())
    }
}

impl Plugins {
    /// Create a new empty set of plugins.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a new plugin to this set.
    pub fn add<T: Plugin + Clone>(&mut self, plugin: T) {
        if self.map.is_none() {
            self.map = Some(HashMap::with_hasher(BuildHasherDefault::default()));
        }
        self.map
            .as_mut()
            .unwrap()
            .insert(TypeId::of::<T::State>(), Box::new(plugin));
    }

    /// Build our plugin states from this set of plugins. Note that if you're
    /// using `azalea` you'll probably never need to use this as it's called
    /// for you.
    pub fn build(self) -> PluginStates {
        let mut map = HashMap::with_hasher(BuildHasherDefault::default());
        for (id, plugin) in self.map.unwrap().into_iter() {
            map.insert(id, plugin.build());
        }
        PluginStates { map: Some(map) }
    }
}

impl IntoIterator for PluginStates {
    type Item = Box<dyn PluginState>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    /// Iterate over the plugin states.
    fn into_iter(self) -> Self::IntoIter {
        self.map
            .map(|map| map.into_values().collect::<Vec<_>>())
            .unwrap_or_default()
            .into_iter()
    }
}

/// A `PluginState` keeps the current state of a plugin for a client. All the
/// fields must be atomic. Unique `PluginState`s are built from [`Plugin`]s.
#[async_trait]
pub trait PluginState: Send + Sync + PluginStateClone + Any + 'static {
    async fn handle(self: Box<Self>, event: Event, bot: Client);
}

/// Plugins can keep their own personal state, listen to [`Event`]s, and add
/// new functions to [`Client`].
pub trait Plugin: Send + Sync + Any + 'static {
    type State: PluginState;

    fn build(&self) -> Self::State;
}

/// AnyPlugin is basically a Plugin but without the State associated type
/// it has to exist so we can do a hashmap with Box<dyn AnyPlugin>
#[doc(hidden)]
pub trait AnyPlugin: Send + Sync + Any + AnyPluginClone + 'static {
    fn build(&self) -> Box<dyn PluginState>;
}

impl<S: PluginState, B: Plugin<State = S> + Clone> AnyPlugin for B {
    fn build(&self) -> Box<dyn PluginState> {
        Box::new(self.build())
    }
}

/// An internal trait that allows PluginState to be cloned.
#[doc(hidden)]
pub trait PluginStateClone {
    fn clone_box(&self) -> Box<dyn PluginState>;
}
impl<T> PluginStateClone for T
where
    T: 'static + PluginState + Clone,
{
    fn clone_box(&self) -> Box<dyn PluginState> {
        Box::new(self.clone())
    }
}
impl Clone for Box<dyn PluginState> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// An internal trait that allows AnyPlugin to be cloned.
#[doc(hidden)]
pub trait AnyPluginClone {
    fn clone_box(&self) -> Box<dyn AnyPlugin>;
}
impl<T> AnyPluginClone for T
where
    T: 'static + Plugin + Clone,
{
    fn clone_box(&self) -> Box<dyn AnyPlugin> {
        Box::new(self.clone())
    }
}
impl Clone for Box<dyn AnyPlugin> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
