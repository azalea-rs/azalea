use std::{
    any::Any,
    borrow::Cow,
    fmt,
    io::{self, Cursor, Write},
};

use azalea_buf::{AzaleaRead, AzaleaReadVar, AzaleaWrite, AzaleaWriteVar, BufReadError};
use azalea_core::codec_utils::is_default;
use azalea_registry::{DataComponentKind, Item};
use indexmap::IndexMap;
use serde::{Serialize, ser::SerializeMap};

use crate::{
    components::{self, DataComponentUnion},
    default_components::get_default_component,
};

/// Either an item in an inventory or nothing.
#[derive(Debug, Clone, Default, PartialEq)]
pub enum ItemStack {
    #[default]
    Empty,
    Present(ItemStackData),
}

impl ItemStack {
    /// Create a new [`ItemStack`] with the given number of [`Item`]s.
    ///
    /// If item is air or the count isn't positive, then it'll be set to an
    /// empty `ItemStack`.
    pub fn new(item: Item, count: i32) -> Self {
        let mut i = ItemStack::Present(ItemStackData::new(item, count));
        // set it to Empty if the item is air or if the count isn't positive
        i.update_empty();
        i
    }

    /// Check if the slot is ItemStack::Empty, if the count is <= 0, or if the
    /// item is air.
    ///
    /// This is the opposite of [`ItemStack::is_present`].
    pub fn is_empty(&self) -> bool {
        match self {
            ItemStack::Empty => true,
            ItemStack::Present(item) => item.is_empty(),
        }
    }
    /// Check if the slot is not ItemStack::Empty, if the count is > 0, and if
    /// the item is not air.
    ///
    /// This is the opposite of [`ItemStack::is_empty`].
    pub fn is_present(&self) -> bool {
        !self.is_empty()
    }

    /// Return the amount of the item in the slot, or 0 if the slot is empty.
    ///
    /// Note that it's possible for the count to be zero or negative when the
    /// slot is present.
    pub fn count(&self) -> i32 {
        match self {
            ItemStack::Empty => 0,
            ItemStack::Present(i) => i.count,
        }
    }

    /// Remove `count` items from this slot, returning the removed items.
    pub fn split(&mut self, count: u32) -> ItemStack {
        match self {
            ItemStack::Empty => ItemStack::Empty,
            ItemStack::Present(i) => {
                let returning = i.split(count);
                if i.is_empty() {
                    *self = ItemStack::Empty;
                }
                ItemStack::Present(returning)
            }
        }
    }

    /// Get the `kind` of the item in this slot, or
    /// [`azalea_registry::Item::Air`]
    pub fn kind(&self) -> azalea_registry::Item {
        match self {
            ItemStack::Empty => azalea_registry::Item::Air,
            ItemStack::Present(i) => i.kind,
        }
    }

    /// Update whether this slot is empty, based on the count.
    pub fn update_empty(&mut self) {
        if let ItemStack::Present(i) = self
            && i.is_empty()
        {
            *self = ItemStack::Empty;
        }
    }

    /// Convert this slot into an [`ItemStackData`], if it's present.
    pub fn as_present(&self) -> Option<&ItemStackData> {
        match self {
            ItemStack::Empty => None,
            ItemStack::Present(i) => Some(i),
        }
    }

    pub fn as_present_mut(&mut self) -> Option<&mut ItemStackData> {
        match self {
            ItemStack::Empty => None,
            ItemStack::Present(i) => Some(i),
        }
    }

    /// Get the value of a data component for this item.
    ///
    /// This is used for things like getting the damage of an item, or seeing
    /// how much food it replenishes.
    pub fn get_component<'a, T: components::DataComponentTrait>(&'a self) -> Option<Cow<'a, T>> {
        self.as_present().and_then(|i| i.get_component::<T>())
    }

    pub fn with_component<
        T: components::EncodableDataComponent + components::DataComponentTrait,
    >(
        mut self,
        component: impl Into<Option<T>>,
    ) -> Self {
        if let ItemStack::Present(i) = &mut self {
            let component: Option<T> = component.into();
            let component: Option<DataComponentUnion> = component.map(|c| c.into());
            i.component_patch.components.insert(T::KIND, component);
        }
        self
    }
}
impl Serialize for ItemStack {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ItemStack::Empty => serializer.serialize_unit(),
            ItemStack::Present(i) => i.serialize(serializer),
        }
    }
}

/// An item in an inventory, with a count and a set of data components.
///
/// Usually you want [`ItemStack`] or [`azalea_registry::Item`] instead.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ItemStackData {
    #[serde(rename = "id")]
    pub kind: azalea_registry::Item,
    /// The amount of the item in this slot.
    ///
    /// The count can be zero or negative, but this is rare.
    pub count: i32,
    /// The item's components that the server set to be different from the
    /// defaults.
    #[serde(rename = "components", skip_serializing_if = "is_default")]
    pub component_patch: DataComponentPatch,
}

impl ItemStackData {
    /// Create a new [`ItemStackData`] with the given number of [`Item`]s.
    pub fn new(item: Item, count: i32) -> Self {
        ItemStackData {
            count,
            kind: item,
            component_patch: DataComponentPatch::default(),
        }
    }

    /// Remove `count` items from this slot, returning the removed items.
    pub fn split(&mut self, count: u32) -> ItemStackData {
        let returning_count = i32::min(count as i32, self.count);
        let mut returning = self.clone();
        returning.count = returning_count;
        self.count -= returning_count;
        returning
    }

    /// Check if the count of the item is <= 0 or if the item is air.
    pub fn is_empty(&self) -> bool {
        self.count <= 0 || self.kind == azalea_registry::Item::Air
    }

    /// Whether this item is the same as another item, ignoring the count.
    ///
    /// ```
    /// # use azalea_inventory::ItemStackData;
    /// # use azalea_registry::Item;
    /// let mut a = ItemStackData::from(Item::Stone);
    /// let mut b = ItemStackData::new(Item::Stone, 2);
    /// assert!(a.is_same_item_and_components(&b));
    ///
    /// b.kind = Item::Dirt;
    /// assert!(!a.is_same_item_and_components(&b));
    /// ```
    pub fn is_same_item_and_components(&self, other: &ItemStackData) -> bool {
        self.kind == other.kind && self.component_patch == other.component_patch
    }

    /// Get the value of a data component for this item.
    ///
    /// This is used for things like getting the damage of an item, or seeing
    /// how much food it replenishes.
    pub fn get_component<'a, T: components::DataComponentTrait>(&'a self) -> Option<Cow<'a, T>> {
        if let Some(c) = self.component_patch.get::<T>() {
            Some(Cow::Borrowed(c))
        } else {
            get_default_component::<T>(self.kind).map(|c| Cow::Owned(c))
        }
    }
}

impl AzaleaRead for ItemStack {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let count = i32::azalea_read_var(buf)?;
        if count <= 0 {
            Ok(ItemStack::Empty)
        } else {
            let kind = azalea_registry::Item::azalea_read(buf)?;
            let component_patch = DataComponentPatch::azalea_read(buf)?;
            Ok(ItemStack::Present(ItemStackData {
                count,
                kind,
                component_patch,
            }))
        }
    }
}

impl AzaleaWrite for ItemStack {
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        match self {
            ItemStack::Empty => 0_i32.azalea_write_var(buf)?,
            ItemStack::Present(i) => {
                i.count.azalea_write_var(buf)?;
                i.kind.azalea_write(buf)?;
                i.component_patch.azalea_write(buf)?;
            }
        };
        Ok(())
    }
}

impl From<ItemStackData> for ItemStack {
    fn from(item: ItemStackData) -> Self {
        if item.is_empty() {
            ItemStack::Empty
        } else {
            ItemStack::Present(item)
        }
    }
}
impl From<Item> for ItemStack {
    fn from(item: Item) -> Self {
        ItemStack::new(item, 1)
    }
}
impl From<(Item, i32)> for ItemStack {
    fn from(item: (Item, i32)) -> Self {
        ItemStack::new(item.0, item.1)
    }
}
impl From<Item> for ItemStackData {
    fn from(item: Item) -> Self {
        ItemStackData::new(item, 1)
    }
}
impl From<(Item, i32)> for ItemStackData {
    fn from(item: (Item, i32)) -> Self {
        ItemStackData::new(item.0, item.1)
    }
}

/// An update to an item's data components.
///
/// Note that in vanilla items come with their own set of default components,
/// and Azalea does not implement that yet.
#[derive(Default)]
pub struct DataComponentPatch {
    components: IndexMap<DataComponentKind, Option<DataComponentUnion>>,
}

impl DataComponentPatch {
    /// Returns the value of the component in the generic argument for this
    /// item.
    ///
    /// ```
    /// # use azalea_inventory::{ItemStackData, DataComponentPatch, components};
    /// # use azalea_registry::Item;
    /// # fn example(item: &ItemStackData) -> Option<()> {
    /// let item_nutrition = item.component_patch.get::<components::Food>()?.nutrition;
    /// # Some(())
    /// # }
    /// ```
    pub fn get<T: components::DataComponentTrait>(&self) -> Option<&T> {
        let component = self.get_kind(T::KIND)?;
        let component_any = component as &dyn Any;
        component_any.downcast_ref::<T>()
    }

    pub fn get_kind(
        &self,
        kind: DataComponentKind,
    ) -> Option<&dyn components::EncodableDataComponent> {
        self.components.get(&kind).and_then(|c| {
            c.as_ref().map(|c| {
                // SAFETY: we just got the component from the map, so it must be the correct
                // kind
                unsafe { c.as_kind(kind) }
            })
        })
    }

    /// Returns whether the component in the generic argument is present for
    /// this item.
    ///
    /// ```
    /// # use azalea_inventory::{ItemStackData, DataComponentPatch, components};
    /// # use azalea_registry::Item;
    /// # let item = ItemStackData::from(Item::Stone);
    /// let is_edible = item.component_patch.has::<components::Food>();
    /// # assert!(!is_edible);
    /// ```
    pub fn has<T: components::DataComponentTrait>(&self) -> bool {
        self.has_kind(T::KIND)
    }

    pub fn has_kind(&self, kind: DataComponentKind) -> bool {
        self.get_kind(kind).is_some()
    }

    pub fn iter<'a>(
        &'a self,
    ) -> impl Iterator<
        Item = (
            DataComponentKind,
            Option<&'a dyn components::EncodableDataComponent>,
        ),
    > + 'a {
        self.components.iter().map(|(&kind, component)| {
            component.as_ref().map_or_else(
                || (kind, None),
                |c| (kind, unsafe { Some(c.as_kind(kind)) }),
            )
        })
    }
    /// Insert a new component into this patch, or mark a component as removed.
    ///
    /// # Safety
    /// The [`DataComponentUnion`] must be of the correct kind.
    pub unsafe fn unchecked_insert_component(
        &mut self,
        kind: DataComponentKind,
        value: Option<DataComponentUnion>,
    ) {
        self.components.insert(kind, value);
    }
}

impl Drop for DataComponentPatch {
    fn drop(&mut self) {
        // the component values are ManuallyDrop since they're in a union
        for (kind, component) in &mut self.components {
            if let Some(component) = component {
                // SAFETY: we got the kind and component from the map
                unsafe { component.drop_as(*kind) };
            }
        }
    }
}

impl AzaleaRead for DataComponentPatch {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let components_with_data_count = u32::azalea_read_var(buf)?;
        let components_without_data_count = u32::azalea_read_var(buf)?;

        if components_without_data_count == 0 && components_with_data_count == 0 {
            return Ok(DataComponentPatch::default());
        }

        let mut components = IndexMap::new();
        for _ in 0..components_with_data_count {
            let component_kind = DataComponentKind::azalea_read(buf)?;
            let component_data = DataComponentUnion::azalea_read_as(component_kind, buf)?;
            components.insert(component_kind, Some(component_data));
        }

        for _ in 0..components_without_data_count {
            let component_kind = DataComponentKind::azalea_read(buf)?;
            components.insert(component_kind, None);
        }

        Ok(DataComponentPatch { components })
    }
}

impl AzaleaWrite for DataComponentPatch {
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        let mut components_with_data_count: u32 = 0;
        let mut components_without_data_count: u32 = 0;
        for component in self.components.values() {
            if component.is_some() {
                components_with_data_count += 1;
            } else {
                components_without_data_count += 1;
            }
        }

        components_with_data_count.azalea_write_var(buf)?;
        components_without_data_count.azalea_write_var(buf)?;

        let mut component_buf = Vec::new();
        for (kind, component) in &self.components {
            if let Some(component) = component {
                kind.azalea_write(buf)?;

                component_buf.clear();
                // SAFETY: we got the component from the map and are passing in the same kind
                unsafe { component.azalea_write_as(*kind, &mut component_buf) }?;
                buf.write_all(&component_buf)?;
            }
        }

        for (kind, component) in &self.components {
            if component.is_none() {
                kind.azalea_write(buf)?;
            }
        }

        Ok(())
    }
}

impl Clone for DataComponentPatch {
    fn clone(&self) -> Self {
        let mut components = IndexMap::with_capacity(self.components.len());
        for (kind, component) in &self.components {
            components.insert(
                *kind,
                component.as_ref().map(|c| unsafe { c.clone_as(*kind) }),
            );
        }
        DataComponentPatch { components }
    }
}
impl fmt::Debug for DataComponentPatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.components.keys()).finish()
    }
}
impl PartialEq for DataComponentPatch {
    fn eq(&self, other: &Self) -> bool {
        if self.components.len() != other.components.len() {
            return false;
        }
        for (kind, component) in &self.components {
            let Some(other_component) = other.components.get(kind) else {
                return false;
            };
            // we can't use PartialEq, but we can use our own eq method
            if let Some(component) = component {
                let Some(other_component) = other_component else {
                    return false;
                };
                // SAFETY: we already checked that the kinds are the same, and we got the
                // components from the map, so they must be the correct kinds
                if !unsafe { component.eq_as(other_component, *kind) } {
                    return false;
                }
            } else if other_component.is_some() {
                return false;
            }
        }
        true
    }
}

impl Serialize for DataComponentPatch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_map(Some(self.components.len()))?;
        for (kind, component) in &self.components {
            if let Some(component) = component {
                unsafe { component.serialize_entry_as(&mut s, *kind) }?;
            } else {
                #[derive(Serialize)]
                struct EmptyComponent;
                s.serialize_entry(&format!("!{kind}"), &EmptyComponent)?;
            }
        }
        s.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::MapId;

    #[test]
    fn test_get_component() {
        let item = ItemStack::from(Item::Map).with_component(MapId { id: 1 });
        let map_id = item.get_component::<MapId>().unwrap();
        assert_eq!(map_id.id, 1);
    }
}
