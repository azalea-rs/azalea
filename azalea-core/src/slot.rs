// TODO: have an azalea-inventory or azalea-container crate and put this there

use azalea_buf::{BufReadError, McBufReadable, McBufVarReadable, McBufVarWritable, McBufWritable};
use azalea_nbt::Tag;
use std::io::{Cursor, Write};

#[derive(Debug, Clone, Default)]
pub enum Slot {
    #[default]
    Empty,
    Present(SlotData),
}

#[derive(Debug, Clone)]
pub struct SlotData {
    pub id: i32,
    pub count: u8,
    pub nbt: Tag,
}

impl McBufReadable for SlotData {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(SlotData {
            id: i32::var_read_from(buf)?,
            count: u8::read_from(buf)?,
            nbt: Tag::read_from(buf)?,
        })
    }
}

impl McBufWritable for SlotData {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        bool::write_into(&true, buf)?;
        self.id.var_write_into(buf)?;
        self.count.write_into(buf)?;
        self.nbt.write_into(buf)?;
        Ok(())
    }
}

impl McBufReadable for Slot {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let present = bool::read_from(buf)?;
        if !present {
            return Ok(Slot::Empty);
        }
        let slot = SlotData::read_from(buf)?;
        Ok(Slot::Present(slot))
    }
}

impl McBufWritable for Slot {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match self {
            Slot::Empty => bool::write_into(&false, buf)?,
            Slot::Present(i) => i.write_into(buf)?,
        }

        Ok(())
    }
}
