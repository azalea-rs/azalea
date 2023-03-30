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

    pub fn count(&self) -> u8 {
        match self {
            ItemSlot::Empty => 0,
            ItemSlot::Present(i) => i.count,
        }
    }
}

/// An item in an inventory, with a count and NBT. Usually you want [`ItemSlot`]
/// or [`azalea_registry::Item`] instead.
#[derive(Debug, Clone, McBuf, PartialEq)]
pub struct ItemSlotData {
    pub kind: azalea_registry::Item,
    pub count: u8,
    pub nbt: Nbt,
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
