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

/// A map of plugin ids to PluginBuilder objects. This can then be built into a
/// [`Plugins`] object as much as you want.
///
/// If you're using azalea, you should generate this from the `plugins!` macro.
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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<T: Plugin + Clone>(&mut self, plugin: T) {
        if self.map.is_none() {
            self.map = Some(HashMap::with_hasher(BuildHasherDefault::default()));
        }
        self.map
            .as_mut()
            .unwrap()
            .insert(TypeId::of::<T::State>(), Box::new(plugin));
    }

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

    fn into_iter(self) -> Self::IntoIter {
        self.map
            .map(|map| map.into_values().collect::<Vec<_>>())
            .unwrap_or_default()
            .into_iter()
    }
}

/// Plugins can keep their own personal state, listen to events, and add new functions to Client.
#[async_trait]
pub trait PluginState: Send + Sync + PluginStateClone + Any + 'static {
    async fn handle(self: Box<Self>, event: Event, bot: Client);
}

/// Plugins can keep their own personal state, listen to events, and add new functions to Client.
pub trait Plugin: Send + Sync + Any + 'static {
    type State: PluginState;

    fn build(&self) -> Box<dyn PluginState>;
}

// AnyPlugin is basically a Plugin but without the State associated type
// it has to exist so we can have a Vec<>
pub trait AnyPlugin: Send + Sync + Any + AnyPluginClone + 'static {
    fn build(&self) -> Box<dyn PluginState>;
}

impl<A, B: Plugin<State = A> + Clone> AnyPlugin for B {
    fn build(&self) -> Box<dyn PluginState> {
        self.build()
    }
}

/// An internal trait that allows Plugin to be cloned.
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
