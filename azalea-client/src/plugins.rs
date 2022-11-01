use crate::{Client, Event};
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
pub struct Plugins {
    map: Option<HashMap<TypeId, Box<dyn Plugin>, BuildHasherDefault<NoHashHasher<u64>>>>,
}

impl Plugins {
    pub fn new() -> Self {
        Self { map: None }
    }

    pub fn add<T: Plugin>(&mut self, plugin: T) {
        if self.map.is_none() {
            self.map = Some(HashMap::with_hasher(BuildHasherDefault::default()));
        }
        self.map
            .as_mut()
            .unwrap()
            .insert(TypeId::of::<T>(), Box::new(plugin));
    }

    pub fn get<T: Plugin>(&self) -> Option<&T> {
        self.map
            .as_ref()
            .and_then(|map| map.get(&TypeId::of::<T>()))
            .and_then(|boxed| (&*boxed as &(dyn Any + 'static)).downcast_ref())
    }
}

impl IntoIterator for Plugins {
    type Item = Box<dyn Plugin>;
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
pub trait Plugin: Send + Sync + PluginClone + 'static {
    async fn handle(self: Box<Self>, event: Event, bot: Client);
}

/// An internal trait that allows Plugin to be cloned.
#[doc(hidden)]
pub trait PluginClone {
    fn clone_box(&self) -> Box<dyn Plugin>;
}
impl<T> PluginClone for T
where
    T: 'static + Plugin + Clone,
{
    fn clone_box(&self) -> Box<dyn Plugin> {
        Box::new(self.clone())
    }
}
impl Clone for Box<dyn Plugin> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
