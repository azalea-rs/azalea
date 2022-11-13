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
pub struct SwarmPlugins {
    map: Option<HashMap<TypeId, Box<dyn SwarmPlugin>, BuildHasherDefault<NoHashHasher<u64>>>>,
}

impl SwarmPlugins {
    pub fn new() -> Self {
        Self { map: None }
    }

    pub fn add<T: SwarmPlugin>(&mut self, plugin: T) {
        if self.map.is_none() {
            self.map = Some(HashMap::with_hasher(BuildHasherDefault::default()));
        }
        self.map
            .as_mut()
            .unwrap()
            .insert(TypeId::of::<T>(), Box::new(plugin));
    }

    pub fn get<T: SwarmPlugin>(&self) -> Option<&T> {
        self.map
            .as_ref()
            .and_then(|map| map.get(&TypeId::of::<T>()))
            .and_then(|boxed| (boxed.as_ref() as &dyn Any).downcast_ref::<T>())
    }
}

impl IntoIterator for SwarmPlugins {
    type Item = Box<dyn SwarmPlugin>;
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
pub trait SwarmPlugin: Send + Sync + SwarmPluginClone + Any + 'static {
    async fn handle(self: Box<Self>, event: SwarmEvent, swarm: Swarm);
}

/// An internal trait that allows Plugin to be cloned.
#[doc(hidden)]
pub trait SwarmPluginClone {
    fn clone_box(&self) -> Box<dyn SwarmPlugin>;
}
impl<T> SwarmPluginClone for T
where
    T: 'static + SwarmPlugin + Clone,
{
    fn clone_box(&self) -> Box<dyn SwarmPlugin> {
        Box::new(self.clone())
    }
}
impl Clone for Box<dyn SwarmPlugin> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
