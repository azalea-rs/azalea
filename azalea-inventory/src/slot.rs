use azalea_buf::{BufReadError, McBuf, McBufReadable, McBufWritable};
use simdnbt::owned::Nbt;
use std::io::{Cursor, Write};

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
    pub fn count(&self) -> i8 {
        match self {
            ItemSlot::Empty => 0,
            ItemSlot::Present(i) => i.count,
        }
    }

    /// Remove `count` items from this slot, returning the removed items.
    pub fn split(&mut self, count: u8) -> ItemSlot {
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
}

/// An item in an inventory, with a count and NBT. Usually you want [`ItemSlot`]
/// or [`azalea_registry::Item`] instead.
#[derive(Debug, Clone, McBuf, PartialEq)]
pub struct ItemSlotData {
    pub kind: azalea_registry::Item,
    /// The amount of the item in this slot.
    ///
    /// The count can be zero or negative, but this is rare.
    pub count: i8,
    pub nbt: Nbt,
}

impl ItemSlotData {
    /// Remove `count` items from this slot, returning the removed items.
    pub fn split(&mut self, count: u8) -> ItemSlotData {
        let returning_count = i8::min(count as i8, self.count);
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
    ///    nbt: Default::default(),
    /// };
    /// let mut b = ItemSlotData {
    ///   kind: Item::Stone,
    ///   count: 2,
    ///   nbt: Default::default(),
    /// };
    /// assert!(a.is_same_item_and_nbt(&b));
    ///
    /// b.kind = Item::Dirt;
    /// assert!(!a.is_same_item_and_nbt(&b));
    /// ```
    pub fn is_same_item_and_nbt(&self, other: &ItemSlotData) -> bool {
        self.kind == other.kind && self.nbt == other.nbt
    }
}

impl McBufReadable for ItemSlot {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let slot = Option::<ItemSlotData>::read_from(buf)?;
        Ok(slot.map_or(ItemSlot::Empty, ItemSlot::Present))
    }
}

impl McBufWritable for ItemSlot {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match self {
            ItemSlot::Empty => false.write_into(buf)?,
            ItemSlot::Present(i) => {
                true.write_into(buf)?;
                i.write_into(buf)?;
            }
        };
        Ok(())
    }
}
