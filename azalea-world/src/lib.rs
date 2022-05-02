use azalea_protocol::mc_buf::{McBufReadable, McBufWritable, Readable, Writable};
use std::io::{Read, Write};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

const SECTION_HEIGHT: u32 = 16;

pub struct World {
    
}

pub struct Chunk {
    pub sections: Vec<Section>,
}

impl Chunk {
    pub fn read_with_world_height(buf: &mut impl Read, world_height: u32) -> Result<Self, String> {
        let section_count = world_height / SECTION_HEIGHT;
        let mut sections = Vec::with_capacity(section_count as usize);
        for _ in 0..section_count {
            let section = Section::read_into(buf)?;
            sections.push(section);
        }
        Ok(Chunk { sections })
    }
}

impl McBufWritable for Chunk {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        for section in &self.sections {
            section.write_into(buf)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Section {
    pub block_count: u16,
    pub states: PalettedContainer,
    pub biomes: PalettedContainer,
}

impl McBufReadable for Section {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        let block_count = u16::read_into(buf)?;
        let states = PalettedContainer::read_into(buf)?;
        let biomes = PalettedContainer::read_into(buf)?;
        Ok(Section {
            block_count,
            states,
            biomes,
        })
    }
}

impl McBufWritable for Section {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        self.block_count.write_into(buf)?;
        self.states.write_into(buf)?;
        self.biomes.write_into(buf)?;
        Ok(())
    }
}

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
