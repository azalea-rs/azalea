use azalea_protocol::mc_buf::{McBufReadable, McBufVarReadable, McBufWritable, Readable, Writable};
use std::io::{Read, Write};

use crate::BitStorage;

#[derive(Clone, Debug, Copy)]
pub enum PalettedContainerType {
    Biomes,
    BlockStates,
}

#[derive(Clone, Debug)]
pub struct PalettedContainer {
    pub bits_per_entry: u8,
    pub palette: Palette,
    /// Compacted list of indices pointing to entry IDs in the Palette.
    pub storage: BitStorage,
    pub container_type: PalettedContainerType,
}

impl PalettedContainer {
    pub fn read_with_type(
        buf: &mut impl Read,
        type_: &'static PalettedContainerType,
    ) -> Result<Self, String> {
        let bits_per_entry = buf.read_byte()?;
        let palette = match type_ {
            PalettedContainerType::BlockStates => {
                Palette::block_states_read_with_bits_per_entry(buf, bits_per_entry)?
            }
            PalettedContainerType::Biomes => {
                Palette::biomes_read_with_bits_per_entry(buf, bits_per_entry)?
            }
        };
        let size = match type_ {
            PalettedContainerType::BlockStates => 4096,
            PalettedContainerType::Biomes => 64,
        };

        let data = Vec::<u64>::read_into(buf)?;
        debug_assert!(
            bits_per_entry != 0 || data.is_empty(),
            "Bits per entry is 0 but data is not empty."
        );
        let storage = BitStorage::new(bits_per_entry.into(), size, Some(data)).unwrap();

        Ok(PalettedContainer {
            bits_per_entry,
            palette,
            storage,
            container_type: *type_,
        })
    }

    pub fn get_index(&self, x: usize, y: usize, z: usize) -> usize {
        let size_bits = match self.container_type {
            PalettedContainerType::BlockStates => 4,
            PalettedContainerType::Biomes => 2,
        };

        (((y << size_bits) | z) << size_bits) | x
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> u32 {
        println!(
            "get: {} {} {}, bits per entry: {}",
            x, y, z, self.bits_per_entry
        );
        let paletted_value = self.storage.get(self.get_index(x, y, z));
        self.palette.value_for(paletted_value as usize)
    }
}

impl McBufWritable for PalettedContainer {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        buf.write_byte(self.bits_per_entry)?;
        self.palette.write_into(buf)?;
        self.storage.data.write_into(buf)?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum Palette {
    /// ID of the corresponding entry in its global palette
    SingleValue(u32),
    Linear(Vec<u32>),
    Hashmap(Vec<u32>),
    Global,
}

impl Palette {
    pub fn block_states_read_with_bits_per_entry(
        buf: &mut impl Read,
        bits_per_entry: u8,
    ) -> Result<Palette, String> {
        Ok(match bits_per_entry {
            0 => Palette::SingleValue(u32::var_read_into(buf)?),
            1..=4 => Palette::Linear(Vec::<u32>::var_read_into(buf)?),
            5..=8 => Palette::Hashmap(Vec::<u32>::var_read_into(buf)?),
            _ => Palette::Global,
        })
    }

    pub fn biomes_read_with_bits_per_entry(
        buf: &mut impl Read,
        bits_per_entry: u8,
    ) -> Result<Palette, String> {
        Ok(match bits_per_entry {
            0 => Palette::SingleValue(u32::var_read_into(buf)?),
            1..=3 => Palette::Linear(Vec::<u32>::var_read_into(buf)?),
            _ => Palette::Global,
        })
    }

    pub fn value_for(&self, value: usize) -> u32 {
        match self {
            Palette::SingleValue(v) => *v,
            Palette::Linear(v) => v[value],
            Palette::Hashmap(v) => v[value],
            Palette::Global => value as u32,
        }
    }
}

impl McBufWritable for Palette {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match self {
            Palette::SingleValue(value) => {
                value.write_into(buf)?;
            }
            Palette::Linear(values) => {
                values.write_into(buf)?;
            }
            Palette::Hashmap(values) => {
                values.write_into(buf)?;
            }
            Palette::Global => {}
        }
        Ok(())
    }
}
