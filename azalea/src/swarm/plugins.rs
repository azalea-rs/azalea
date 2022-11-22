use crate::{Client, Event, Swarm, SwarmEvent};
use async_trait::async_trait;
use nohash_hasher::NoHashHasher;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    hash::BuildHasherDefault,
};

// kind of based on https://docs.rs/http/latest/src/http/extensions.rs.html
/// A map of plugin ids to Plugin trait objects. The client stores this so we
/// can keep the state for our plugins.
///
/// If you're using azalea, you should generate this from the `plugins!` macro.
#[derive(Clone)]
pub struct SwarmPlugins<S> {
    map: Option<HashMap<TypeId, Box<dyn SwarmPlugin<S>>, BuildHasherDefault<NoHashHasher<u64>>>>,
}

#[derive(Clone)]
pub struct SwarmPluginStates<S> {
    map: Option<
        HashMap<TypeId, Box<dyn SwarmPluginState<S>>, BuildHasherDefault<NoHashHasher<u64>>>,
    >,
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
    pub fn new() -> Self {
        Self { map: None }
    }

    pub fn add<T: SwarmPlugin<S>>(&mut self, plugin: T) {
        if self.map.is_none() {
            self.map = Some(HashMap::with_hasher(BuildHasherDefault::default()));
        }
        self.map
            .as_mut()
            .unwrap()
            .insert(TypeId::of::<T>(), Box::new(plugin));
    }

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

    fn into_iter(self) -> Self::IntoIter {
        self.map
            .map(|map| map.into_iter().map(|(_, v)| v).collect::<Vec<_>>())
            .unwrap_or_default()
            .into_iter()
    }
}

/// Plugins can keep their own personal state, listen to events, and add new functions to Client.
#[async_trait]
pub trait SwarmPluginState<S>: Send + Sync + SwarmPluginStateClone<S> + Any + 'static {
    async fn handle(self: Box<Self>, event: SwarmEvent, swarm: Swarm<S>);
}

#[async_trait]
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
