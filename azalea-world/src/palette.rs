use std::io::{Read, Write};

use azalea_protocol::mc_buf::{McBufReadable, McBufWritable, Readable, Writable};

#[derive(Clone, Debug)]
pub struct PalettedContainer {
    pub bits_per_entry: u8,
    pub palette: Palette,
    /// Compacted list of indices pointing to entry IDs in the Palette.
    pub data: Vec<i64>,
}

impl McBufReadable for PalettedContainer {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        let bits_per_entry = buf.read_byte()?;
        let palette = Palette::read_with_bits_per_entry(buf, bits_per_entry)?;
        let data = Vec::<i64>::read_into(buf)?;
        Ok(PalettedContainer {
            bits_per_entry,
            palette,
            data,
        })
    }
}
impl McBufWritable for PalettedContainer {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        buf.write_byte(self.bits_per_entry)?;
        self.palette.write_into(buf)?;
        self.data.write_into(buf)?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum Palette {
    /// ID of the corresponding entry in its global palette
    SingleValue(u32),
    LinearPalette(Vec<u32>),
    HashmapPalette(Vec<u32>),
    GlobalPalette,
}

impl Palette {
    pub fn read_with_bits_per_entry(
        buf: &mut impl Read,
        bits_per_entry: u8,
    ) -> Result<Palette, String> {
        Ok(match bits_per_entry {
            0 => Palette::SingleValue(u32::read_into(buf)?),
            1..=4 => Palette::LinearPalette(Vec::<u32>::read_into(buf)?),
            5..=8 => Palette::HashmapPalette(Vec::<u32>::read_into(buf)?),
            _ => Palette::GlobalPalette,
        })
    }
}

impl McBufWritable for Palette {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match self {
            Palette::SingleValue(value) => {
                value.write_into(buf)?;
            }
            Palette::LinearPalette(values) => {
                values.write_into(buf)?;
            }
            Palette::HashmapPalette(values) => {
                values.write_into(buf)?;
            }
            Palette::GlobalPalette => {}
        }
        Ok(())
    }
}
