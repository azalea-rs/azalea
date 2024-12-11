use std::{
    any::Any,
    collections::HashMap,
    fmt,
    io::{Cursor, Write},
};

use azalea_buf::{AzaleaRead, AzaleaReadVar, AzaleaWrite, AzaleaWriteVar, BufReadError};
use azalea_registry::DataComponentKind;

use crate::components::{self};

/// Either an item in an inventory or nothing.
#[derive(Debug, Clone, Default, PartialEq)]
pub enum ItemStack {
    #[default]
    Empty,
    Present(ItemStackData),
}

impl ItemStack {
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
        if let ItemStack::Present(i) = self {
            if i.is_empty() {
                *self = ItemStack::Empty;
            }
        }
    }

    /// Convert this slot into an [`ItemStackData`], if it's present.
    pub fn as_present(&self) -> Option<&ItemStackData> {
        match self {
            ItemStack::Empty => None,
            ItemStack::Present(i) => Some(i),
        }
    }
}

/// An item in an inventory, with a count and NBT. Usually you want
/// [`ItemStack`] or [`azalea_registry::Item`] instead.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemStackData {
    /// The amount of the item in this slot.
    ///
    /// The count can be zero or negative, but this is rare.
    pub count: i32,
    pub kind: azalea_registry::Item,
    pub components: DataComponentPatch,
}

impl ItemStackData {
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
    /// let mut a = ItemStackData {
    ///    kind: Item::Stone,
    ///    count: 1,
    ///    components: Default::default(),
    /// };
    /// let mut b = ItemStackData {
    ///   kind: Item::Stone,
    ///   count: 2,
    ///   components: Default::default(),
    /// };
    /// assert!(a.is_same_item_and_components(&b));
    ///
    /// b.kind = Item::Dirt;
    /// assert!(!a.is_same_item_and_components(&b));
    /// ```
    pub fn is_same_item_and_components(&self, other: &ItemStackData) -> bool {
        self.kind == other.kind && self.components == other.components
    }
}

impl AzaleaRead for ItemStack {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let count = i32::azalea_read_var(buf)?;
        if count <= 0 {
            Ok(ItemStack::Empty)
        } else {
            let kind = azalea_registry::Item::azalea_read(buf)?;
            let components = DataComponentPatch::azalea_read(buf)?;
            Ok(ItemStack::Present(ItemStackData {
                count,
                kind,
                components,
            }))
        }
    }
}

impl AzaleaWrite for ItemStack {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match self {
            ItemStack::Empty => 0_i32.azalea_write_var(buf)?,
            ItemStack::Present(i) => {
                i.count.azalea_write_var(buf)?;
                i.kind.azalea_write(buf)?;
                i.components.azalea_write(buf)?;
            }
        };
        Ok(())
    }
}

/// An update to an item's data components.
///
/// Note that in vanilla items come with their own set of default components,
/// and Azalea does not implement that yet.
#[derive(Default)]
pub struct DataComponentPatch {
    components: HashMap<DataComponentKind, Option<Box<dyn components::EncodableDataComponent>>>,
}

impl DataComponentPatch {
    /// Returns the value of the component in the generic argument for this
    /// item.
    ///
    /// ```
    /// # use azalea_inventory::{ItemStackData, DataComponentPatch, components};
    /// # use azalea_registry::Item;
    /// # fn example(item: &ItemStackData) -> Option<()> {
    /// let item_nutrition = item.components.get::<components::Food>()?.nutrition;
    /// # Some(())
    /// # }
    /// ```
    pub fn get<T: components::DataComponent>(&self) -> Option<&T> {
        let component = self.components.get(&T::KIND).and_then(|c| c.as_deref())?;
        let component_any = component as &dyn Any;
        component_any.downcast_ref::<T>()
    }

    pub fn get_kind(
        &self,
        kind: DataComponentKind,
    ) -> Option<&dyn components::EncodableDataComponent> {
        self.components.get(&kind).and_then(|c| c.as_deref())
    }

    /// Returns whether the component in the generic argument is present for
    /// this item.
    ///
    /// ```
    /// # use azalea_inventory::{ItemStackData, DataComponentPatch, components};
    /// # use azalea_registry::Item;
    /// # let item = ItemStackData {
    /// #     kind: Item::Stone,
    /// #     count: 1,
    /// #     components: Default::default(),
    /// # };
    /// let is_edible = item.components.has::<components::Food>();
    /// # assert!(!is_edible);
    /// ```
    pub fn has<T: components::DataComponent>(&self) -> bool {
        self.has_kind(T::KIND)
    }

    pub fn has_kind(&self, kind: DataComponentKind) -> bool {
        self.get_kind(kind).is_some()
    }
}

impl AzaleaRead for DataComponentPatch {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let components_with_data_count = u32::azalea_read_var(buf)?;
        let components_without_data_count = u32::azalea_read_var(buf)?;

        if components_without_data_count == 0 && components_with_data_count == 0 {
            return Ok(DataComponentPatch::default());
        }

        let mut components = HashMap::new();
        for _ in 0..components_with_data_count {
            let component_kind = DataComponentKind::azalea_read(buf)?;
            let component_data = components::from_kind(component_kind, buf)?;
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
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let mut components_with_data_count = 0;
        let mut components_without_data_count = 0;
        for component in self.components.values() {
            if component.is_some() {
                components_with_data_count += 1;
            } else {
                components_without_data_count += 1;
            }
        }

        components_with_data_count.azalea_write_var(buf)?;
        components_without_data_count.azalea_write_var(buf)?;

        for (kind, component) in &self.components {
            if let Some(component) = component {
                kind.azalea_write(buf)?;
                let mut component_buf = Vec::new();
                component.encode(&mut component_buf).unwrap();
                component_buf.azalea_write(buf)?;
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
        let mut components = HashMap::with_capacity(self.components.len());
        for (kind, component) in &self.components {
            components.insert(*kind, component.as_ref().map(|c| (*c).clone()));
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
            if let Some(other_component) = other.components.get(kind) {
                // we can't use PartialEq, but we can use our own eq method
                if let Some(component) = component {
                    if let Some(other_component) = other_component {
                        if !component.eq((*other_component).clone()) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                } else if other_component.is_some() {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}
