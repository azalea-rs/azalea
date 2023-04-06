use azalea_buf::{BufReadError, McBuf, McBufReadable, McBufWritable};
use azalea_nbt::Nbt;
use std::io::{Cursor, Write};

/// Either an item in an inventory or nothing.
#[derive(Debug, Clone, Default, PartialEq)]
pub enum ItemSlot {
    #[default]
    Empty,
    Present(ItemSlotData),
}

impl ItemSlot {
    pub fn is_empty(&self) -> bool {
        matches!(self, ItemSlot::Empty)
    }
    pub fn is_present(&self) -> bool {
        matches!(self, ItemSlot::Present(_))
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
        if count == 0 {
            return ItemSlot::Empty;
        }
        match self {
            ItemSlot::Empty => ItemSlot::Empty,
            ItemSlot::Present(i) => {
                let returning = i.split(count);
                if i.count == 0 {
                    *self = ItemSlot::Empty;
                }
                ItemSlot::Present(returning)
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
