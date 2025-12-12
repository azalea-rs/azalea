use std::{collections::HashSet, ops::Deref};

use crate::Registry;

pub mod blocks;
pub mod entities;
pub mod fluids;
pub mod items;

/// A set of registry items.
#[derive(Clone, Debug)]
pub struct RegistryTag<R: Registry + 'static> {
    // yes, this *could* be a Box<[R]>, but there are cases in which the user may want to cheaply
    // mutate this.
    // yes, it could also be a [R; N] using a const generic, but that makes RegistryTag annoying if
    // the user wants to put it in their own code.
    // yes, if the aforementioned issues are ignored then we could even avoid a LazyLock. but that
    // would provide nearly no benefit and result in wasted memory for unused tags.
    // having it be a vec is fine.
    entries: Vec<R>,
}
impl<R: Registry + 'static> RegistryTag<R> {
    pub(crate) fn new(entries: Vec<R>) -> Self {
        // must be sorted for binary search
        debug_assert!(entries.is_sorted());

        Self { entries }
    }
}
impl<R: Registry + 'static> RegistryTag<R> {
    /// Returns whether the given item is contained in this registry.
    pub fn contains(&self, value: &R) -> bool {
        self.find(value).is_some()
    }

    pub fn remove(&mut self, value: &R) -> Option<R> {
        self.find(value).map(|index| self.entries.remove(index))
    }

    fn find(&self, value: &R) -> Option<usize> {
        // TODO: tune this number; when does binary search actually start making a
        // difference?
        if self.entries.len() > 64 {
            self.linear_search_find(value)
        } else {
            self.binary_search_find(value)
        }
    }
    fn linear_search_find(&self, value: &R) -> Option<usize> {
        self.entries.iter().position(|e| e == value)
    }
    fn binary_search_find(&self, value: &R) -> Option<usize> {
        self.entries.binary_search(value).ok()
    }

    // this exists for convenience since we can't always do HashSet::from if it's a
    // LazyLock
    pub fn into_hashset(&self) -> HashSet<R> {
        self.clone().into_iter().collect()
    }
}
impl<R: Registry> IntoIterator for RegistryTag<R> {
    type Item = R;

    type IntoIter = std::vec::IntoIter<R>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

impl<R: Registry> From<RegistryTag<R>> for HashSet<R> {
    fn from(tag: RegistryTag<R>) -> Self {
        tag.into_hashset()
    }
}

impl<R: Registry> Deref for RegistryTag<R> {
    type Target = [R];

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

impl<R: Registry> FromIterator<R> for RegistryTag<R> {
    fn from_iter<T: IntoIterator<Item = R>>(iter: T) -> Self {
        let mut entries = iter.into_iter().collect::<Vec<_>>();
        entries.sort();
        Self::new(entries)
    }
}
