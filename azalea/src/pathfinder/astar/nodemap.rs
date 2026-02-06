use std::hash::{BuildHasherDefault, Hash};

use indexmap::{IndexMap, map::Entry};
use rustc_hash::FxHasher;

use crate::pathfinder::astar::Node;

type FxIndexMap<K, V> = IndexMap<K, V, BuildHasherDefault<FxHasher>>;

pub struct NodeMap<P> {
    map: FxIndexMap<P, Node>,
}
impl<P: Hash + Eq> NodeMap<P> {
    pub fn insert(&mut self, key: P, value: Node) {
        self.map.insert(key, value);
    }
    pub fn get_index(&self, index: u32) -> Option<(&P, &Node)> {
        self.map.get_index(index as usize)
    }
    pub fn entry(&mut self, key: P) -> Entry<'_, P, Node> {
        self.map.entry(key)
    }
}
impl<P> Default for NodeMap<P> {
    fn default() -> Self {
        Self {
            map: Default::default(),
        }
    }
}
