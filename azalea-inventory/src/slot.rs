use azalea_buf::{BufReadError, McBufReadable, McBufVarReadable, McBufVarWritable, McBufWritable};
use azalea_registry::DataComponentKind;
use std::{
    collections::HashMap,
    fmt,
    io::{Cursor, Write},
};

use crate::components::{self};

/// Either an item in an inventory or nothing.
#[derive(Debug, Clone, Default, PartialEq)]
pub enum ItemSlot {
    #[default]
    Empty,
    Present(ItemSlotData),
}

impl ItemSlot {
    /// Check if the slot is ItemSlot::Empty, if the count is <= 0, or if the
    /// item is air.
    ///
    /// This is the opposite of [`ItemSlot::is_present`].
    pub fn is_empty(&self) -> bool {
        match self {
            ItemSlot::Empty => true,
            ItemSlot::Present(item) => item.is_empty(),
        }
    }
    /// Check if the slot is not ItemSlot::Empty, if the count is > 0, and if
    /// the item is not air.
    ///
    /// This is the opposite of [`ItemSlot::is_empty`].
    pub fn is_present(&self) -> bool {
        !self.is_empty()
    }

    /// Return the amount of the item in the slot, or 0 if the slot is empty.
    ///
    /// Note that it's possible for the count to be zero or negative when the
    /// slot is present.
    pub fn count(&self) -> i32 {
        match self {
            ItemSlot::Empty => 0,
            ItemSlot::Present(i) => i.count,
        }
    }

    /// Remove `count` items from this slot, returning the removed items.
    pub fn split(&mut self, count: u32) -> ItemSlot {
        match self {
            ItemSlot::Empty => ItemSlot::Empty,
            ItemSlot::Present(i) => {
                let returning = i.split(count);
                if i.is_empty() {
                    *self = ItemSlot::Empty;
                }
                ItemSlot::Present(returning)
            }
        }
    }

    /// Get the `kind` of the item in this slot, or
    /// [`azalea_registry::Item::Air`]
    pub fn kind(&self) -> azalea_registry::Item {
        match self {
            ItemSlot::Empty => azalea_registry::Item::Air,
            ItemSlot::Present(i) => i.kind,
        }
    }

    /// Update whether this slot is empty, based on the count.
    pub fn update_empty(&mut self) {
        if let ItemSlot::Present(i) = self {
            if i.is_empty() {
                *self = ItemSlot::Empty;
            }
        }
    }

    /// Convert this slot into an [`ItemSlotData`], if it's present.
    pub fn as_present(&self) -> Option<&ItemSlotData> {
        match self {
            ItemSlot::Empty => None,
            ItemSlot::Present(i) => Some(i),
        }
    }
}

/// An item in an inventory, with a count and NBT. Usually you want [`ItemSlot`]
/// or [`azalea_registry::Item`] instead.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemSlotData {
    /// The amount of the item in this slot.
    ///
    /// The count can be zero or negative, but this is rare.
    pub count: i32,
    pub kind: azalea_registry::Item,
    pub components: DataComponentPatch,
}

impl ItemSlotData {
    /// Remove `count` items from this slot, returning the removed items.
    pub fn split(&mut self, count: u32) -> ItemSlotData {
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
    /// # use azalea_inventory::ItemSlotData;
    /// # use azalea_registry::Item;
    /// let mut a = ItemSlotData {
    ///    kind: Item::Stone,
    ///    count: 1,
    ///    components: Default::default(),
    /// };
    /// let mut b = ItemSlotData {
    ///   kind: Item::Stone,
    ///   count: 2,
    ///   components: Default::default(),
    /// };
    /// assert!(a.is_same_item_and_components(&b));
    ///
    /// b.kind = Item::Dirt;
    /// assert!(!a.is_same_item_and_components(&b));
    /// ```
    pub fn is_same_item_and_components(&self, other: &ItemSlotData) -> bool {
        self.kind == other.kind && self.components == other.components
    }
}

impl McBufReadable for ItemSlot {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let count = i32::var_read_from(buf)?;
        if count <= 0 {
            Ok(ItemSlot::Empty)
        } else {
            let kind = azalea_registry::Item::read_from(buf)?;
            let components = DataComponentPatch::read_from(buf)?;
            Ok(ItemSlot::Present(ItemSlotData {
                count,
                kind,
                components,
            }))
        }
    }
}

impl McBufWritable for ItemSlot {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match self {
            ItemSlot::Empty => 0.var_write_into(buf)?,
            ItemSlot::Present(i) => {
                i.count.var_write_into(buf)?;
                i.kind.write_into(buf)?;
                i.components.write_into(buf)?;
            }
        };
        Ok(())
    }
}

#[derive(Default)]
pub struct DataComponentPatch {
    components: HashMap<DataComponentKind, Option<Box<dyn components::EncodableDataComponent>>>,
}

impl DataComponentPatch {
    pub fn get(&self, kind: DataComponentKind) -> Option<&dyn components::EncodableDataComponent> {
        self.components.get(&kind).and_then(|c| c.as_deref())
    }
}

impl McBufReadable for DataComponentPatch {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let components_with_data_count = u32::var_read_from(buf)?;
        let components_without_data_count = u32::var_read_from(buf)?;

        if components_without_data_count == 0 && components_with_data_count == 0 {
            return Ok(DataComponentPatch::default());
        }

        let mut components = HashMap::new();
        for _ in 0..components_with_data_count {
            let component_kind = DataComponentKind::read_from(buf)?;
            let component_data = components::from_kind(component_kind, buf)?;
            components.insert(component_kind, Some(component_data));
        }

        for _ in 0..components_without_data_count {
            let component_kind = DataComponentKind::read_from(buf)?;
            components.insert(component_kind, None);
        }

        Ok(DataComponentPatch { components })
    }
}

impl McBufWritable for DataComponentPatch {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let mut components_with_data_count = 0;
        let mut components_without_data_count = 0;
        for component in self.components.values() {
            if component.is_some() {
                components_with_data_count += 1;
            } else {
                components_without_data_count += 1;
            }
        }

        components_with_data_count.write_into(buf)?;
        components_without_data_count.write_into(buf)?;

        for (kind, component) in &self.components {
            if let Some(component) = component {
                kind.write_into(buf)?;
                let mut component_buf = Vec::new();
                component.encode(&mut component_buf).unwrap();
                component_buf.write_into(buf)?;
            }
        }

        for (kind, component) in &self.components {
            if component.is_none() {
                kind.write_into(buf)?;
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
