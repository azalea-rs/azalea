// TODO: have an azalea-inventory or azalea-container crate and put this there

use azalea_buf::{McBuf, McBufReadable, McBufWritable};
use std::io::{Read, Write};

#[derive(Debug, Clone)]
pub enum Slot {
    Empty,
    Present(SlotData),
}

#[derive(Debug, Clone, McBuf)]
pub struct SlotData {
    #[var]
    pub id: i32,
    pub count: u8,
    pub nbt: azalea_nbt::Tag,
}

impl McBufReadable for Slot {
    fn read_from(buf: &mut impl Read) -> Result<Self, String> {
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
            Slot::Empty => 0u8.write_into(buf)?,
            Slot::Present(i) => i.write_into(buf)?,
        }

        Ok(())
    }
}
