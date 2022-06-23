// TODO: have an azalea-inventory or azalea-container crate and put this there

#[derive(Debug, Clone)]
pub enum Slot {
    Present(SlotData),
    Empty,
}

#[derive(Debug, Clone)]
pub struct SlotData {
    pub id: i32,
    pub count: u8,
    pub nbt: azalea_nbt::Tag,
}

impl McBufReadable for Slot {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        let present = buf.read_boolean()?;
        if !present {
            return Ok(Slot::Empty);
        }
        let id = buf.read_varint()?;
        let count = buf.read_byte()?;
        let nbt = buf.read_nbt()?;
        Ok(Slot::Present(SlotData { id, count, nbt }))
    }
}

impl McBufWritable for Slot {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match self {
            Slot::Empty => buf.write_byte(0)?,
            Slot::Present(i) => {
                buf.write_varint(i.id)?;
                buf.write_byte(i.count)?;
                buf.write_nbt(&i.nbt)?;
            }
        }

        Ok(())
    }
}
