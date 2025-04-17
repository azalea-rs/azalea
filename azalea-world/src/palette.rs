use std::io::{Cursor, Write};

use azalea_block::BlockState;
use azalea_buf::{AzaleaRead, AzaleaWrite, BufReadError};
use tracing::warn;

use crate::BitStorage;

#[derive(Clone, Debug, Copy)]
pub enum PalettedContainerKind {
    Biomes,
    BlockStates,
}

#[derive(Clone, Debug)]
pub struct PalettedContainer {
    pub bits_per_entry: u8,
    /// This is usually a list of unique values that appear in the container so
    /// they can be indexed by the bit storage.
    ///
    /// Sometimes it doesn't contain anything if there's too many unique items
    /// in the bit storage, though.
    pub palette: Palette,
    /// Compacted list of indices pointing to entry IDs in the Palette.
    pub storage: BitStorage,
    pub container_type: PalettedContainerKind,
}

impl PalettedContainer {
    pub fn new(container_type: PalettedContainerKind) -> Self {
        let palette = Palette::SingleValue(BlockState::AIR);
        let size = container_type.size();
        let storage = BitStorage::new(0, size, Some(Box::new([]))).unwrap();

        PalettedContainer {
            bits_per_entry: 0,
            palette,
            storage,
            container_type,
        }
    }

    pub fn read_with_type(
        buf: &mut Cursor<&[u8]>,
        container_type: &'static PalettedContainerKind,
    ) -> Result<Self, BufReadError> {
        let bits_per_entry = u8::azalea_read(buf)?;
        let palette_type = PaletteKind::from_bits_and_type(bits_per_entry, container_type);
        let palette = palette_type.read(buf)?;
        let size = container_type.size();

        let mut storage = match BitStorage::new(
            bits_per_entry as usize,
            size,
            if bits_per_entry == 0 {
                Some(Box::new([]))
            } else {
                // we're going to update the data after creating the bitstorage
                None
            },
        ) {
            Ok(storage) => storage,
            Err(e) => {
                warn!("Failed to create bit storage: {:?}", e);
                return Err(BufReadError::Custom(
                    "Failed to create bit storage".to_string(),
                ));
            }
        };

        // now read the data
        for i in 0..storage.data.len() {
            storage.data[i] = u64::azalea_read(buf)?;
        }

        Ok(PalettedContainer {
            bits_per_entry,
            palette,
            storage,
            container_type: *container_type,
        })
    }

    /// Calculates the index of the given coordinates.
    pub fn index_from_coords(&self, x: usize, y: usize, z: usize) -> usize {
        let size_bits = self.container_type.size_bits();

        (((y << size_bits) | z) << size_bits) | x
    }

    pub fn coords_from_index(&self, index: usize) -> (usize, usize, usize) {
        let size_bits = self.container_type.size_bits();
        let mask = (1 << size_bits) - 1;
        (
            index & mask,
            (index >> size_bits >> size_bits) & mask,
            (index >> size_bits) & mask,
        )
    }

    /// Returns the value at the given index.
    ///
    /// # Panics
    ///
    /// This function panics if the index is greater than or equal to the number
    /// of things in the storage. (So for block states, it must be less than
    /// 4096).
    pub fn get_at_index(&self, index: usize) -> BlockState {
        // first get the palette id
        let paletted_value = self.storage.get(index);
        // and then get the value from that id
        self.palette.value_for(paletted_value as usize)
    }

    /// Returns the value at the given coordinates.
    pub fn get(&self, x: usize, y: usize, z: usize) -> BlockState {
        // let paletted_value = self.storage.get(self.get_index(x, y, z));
        // self.palette.value_for(paletted_value as usize)
        self.get_at_index(self.index_from_coords(x, y, z))
    }

    /// Sets the id at the given coordinates and return the previous id
    pub fn get_and_set(&mut self, x: usize, y: usize, z: usize, value: BlockState) -> BlockState {
        let paletted_value = self.id_for(value);
        let block_state_id = self
            .storage
            .get_and_set(self.index_from_coords(x, y, z), paletted_value as u64);
        // error in debug mode
        #[cfg(debug_assertions)]
        if block_state_id > BlockState::MAX_STATE.into() {
            warn!(
                "Old block state from get_and_set {block_state_id} was greater than max state {}",
                BlockState::MAX_STATE
            );
        }

        BlockState::try_from(block_state_id as u32).unwrap_or_default()
    }

    /// Sets the id at the given index and return the previous id. You probably
    /// want `.set` instead.
    pub fn set_at_index(&mut self, index: usize, value: BlockState) {
        let paletted_value = self.id_for(value);
        self.storage.set(index, paletted_value as u64);
    }

    /// Sets the id at the given coordinates and return the previous id
    pub fn set(&mut self, x: usize, y: usize, z: usize, value: BlockState) {
        self.set_at_index(self.index_from_coords(x, y, z), value);
    }

    fn create_or_reuse_data(&self, bits_per_entry: u8) -> PalettedContainer {
        let new_palette_type =
            PaletteKind::from_bits_and_type(bits_per_entry, &self.container_type);

        let old_palette_type = (&self.palette).into();
        if bits_per_entry == self.bits_per_entry && new_palette_type == old_palette_type {
            return self.clone();
        }
        let storage =
            BitStorage::new(bits_per_entry as usize, self.container_type.size(), None).unwrap();

        // sanity check
        debug_assert_eq!(storage.size(), self.container_type.size());

        // let palette = new_palette_type.as_empty_palette(1usize << (bits_per_entry as
        // usize));
        let palette = new_palette_type.as_empty_palette();
        PalettedContainer {
            bits_per_entry,
            palette,
            storage,
            container_type: self.container_type,
        }
    }

    fn on_resize(&mut self, bits_per_entry: u8, value: BlockState) -> usize {
        // in vanilla this is always true, but it's sometimes false in purpur servers
        // assert!(bits_per_entry <= 5, "bits_per_entry must be <= 5");
        let mut new_data = self.create_or_reuse_data(bits_per_entry);
        new_data.copy_from(&self.palette, &self.storage);
        *self = new_data;
        self.id_for(value)
    }

    fn copy_from(&mut self, palette: &Palette, storage: &BitStorage) {
        for i in 0..storage.size() {
            let value = palette.value_for(storage.get(i) as usize);
            let id = self.id_for(value) as u64;
            self.storage.set(i, id);
        }
    }

    pub fn id_for(&mut self, value: BlockState) -> usize {
        match &mut self.palette {
            Palette::SingleValue(v) => {
                if *v != value {
                    self.on_resize(1, value)
                } else {
                    0
                }
            }
            Palette::Linear(palette) => {
                if let Some(index) = palette.iter().position(|&v| v == value) {
                    return index;
                }
                let capacity = 2usize.pow(self.bits_per_entry.into());
                if capacity > palette.len() {
                    palette.push(value);
                    palette.len() - 1
                } else {
                    self.on_resize(self.bits_per_entry + 1, value)
                }
            }
            Palette::Hashmap(palette) => {
                // TODO? vanilla keeps this in memory as a hashmap, but it should be benchmarked
                // before changing it
                if let Some(index) = palette.iter().position(|v| *v == value) {
                    return index;
                }
                let capacity = 2usize.pow(self.bits_per_entry.into());
                if capacity > palette.len() {
                    palette.push(value);
                    palette.len() - 1
                } else {
                    self.on_resize(self.bits_per_entry + 1, value)
                }
            }
            Palette::Global => value.id() as usize,
        }
    }
}

impl AzaleaWrite for PalettedContainer {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        self.bits_per_entry.azalea_write(buf)?;
        self.palette.azalea_write(buf)?;
        self.storage.data.azalea_write(buf)?;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PaletteKind {
    SingleValue,
    Linear,
    Hashmap,
    Global,
}

/// A representation of the different types of chunk palettes Minecraft uses.
#[derive(Clone, Debug)]
pub enum Palette {
    /// ID of the corresponding entry in its global palette
    SingleValue(BlockState),
    // in vanilla this keeps a `size` field that might be less than the length, but i'm not sure
    // it's actually needed?
    Linear(Vec<BlockState>),
    Hashmap(Vec<BlockState>),
    Global,
}

impl Palette {
    pub fn value_for(&self, id: usize) -> BlockState {
        match self {
            Palette::SingleValue(v) => *v,
            Palette::Linear(v) => v.get(id).copied().unwrap_or_default(),
            Palette::Hashmap(v) => v.get(id).copied().unwrap_or_default(),
            Palette::Global => BlockState::try_from(id as u32).unwrap_or_default(),
        }
    }
}

impl AzaleaWrite for Palette {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match self {
            Palette::SingleValue(value) => {
                value.azalea_write(buf)?;
            }
            Palette::Linear(values) => {
                values.azalea_write(buf)?;
            }
            Palette::Hashmap(values) => {
                values.azalea_write(buf)?;
            }
            Palette::Global => {}
        }
        Ok(())
    }
}

impl PaletteKind {
    pub fn from_bits_and_type(bits_per_entry: u8, container_type: &PalettedContainerKind) -> Self {
        match container_type {
            PalettedContainerKind::BlockStates => match bits_per_entry {
                0 => PaletteKind::SingleValue,
                1..=4 => PaletteKind::Linear,
                5..=8 => PaletteKind::Hashmap,
                _ => PaletteKind::Global,
            },
            PalettedContainerKind::Biomes => match bits_per_entry {
                0 => PaletteKind::SingleValue,
                1..=3 => PaletteKind::Linear,
                _ => PaletteKind::Global,
            },
        }
    }

    pub fn read(&self, buf: &mut Cursor<&[u8]>) -> Result<Palette, BufReadError> {
        Ok(match self {
            // since they're read as varints it's actually fine to just use BlockStateIntegerRepr
            // instead of the correct type (u32)
            PaletteKind::SingleValue => Palette::SingleValue(BlockState::azalea_read(buf)?),
            PaletteKind::Linear => Palette::Linear(Vec::<BlockState>::azalea_read(buf)?),
            PaletteKind::Hashmap => Palette::Hashmap(Vec::<BlockState>::azalea_read(buf)?),
            PaletteKind::Global => Palette::Global,
        })
    }

    pub fn as_empty_palette(&self) -> Palette {
        match self {
            PaletteKind::SingleValue => Palette::SingleValue(BlockState::AIR),
            PaletteKind::Linear => Palette::Linear(Vec::new()),
            PaletteKind::Hashmap => Palette::Hashmap(Vec::new()),
            PaletteKind::Global => Palette::Global,
        }
    }
}

impl From<&Palette> for PaletteKind {
    fn from(palette: &Palette) -> Self {
        match palette {
            Palette::SingleValue(_) => PaletteKind::SingleValue,
            Palette::Linear(_) => PaletteKind::Linear,
            Palette::Hashmap(_) => PaletteKind::Hashmap,
            Palette::Global => PaletteKind::Global,
        }
    }
}

impl PalettedContainerKind {
    fn size_bits(&self) -> usize {
        match self {
            PalettedContainerKind::BlockStates => 4,
            PalettedContainerKind::Biomes => 2,
        }
    }

    fn size(&self) -> usize {
        1 << (self.size_bits() * 3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resize_0_bits_to_1() {
        let mut palette_container = PalettedContainer::new(PalettedContainerKind::BlockStates);

        assert_eq!(palette_container.bits_per_entry, 0);
        assert_eq!(palette_container.get_at_index(0), BlockState::AIR);
        assert_eq!(
            PaletteKind::from(&palette_container.palette),
            PaletteKind::SingleValue
        );
        let block_state_1 = BlockState::try_from(1_u32).unwrap();
        palette_container.set_at_index(0, block_state_1);
        assert_eq!(palette_container.get_at_index(0), block_state_1);
        assert_eq!(
            PaletteKind::from(&palette_container.palette),
            PaletteKind::Linear
        );
    }

    #[test]
    fn test_resize_0_bits_to_5() {
        let mut palette_container = PalettedContainer::new(PalettedContainerKind::BlockStates);

        let set = |pc: &mut PalettedContainer, i, v: u32| {
            pc.set_at_index(i, BlockState::try_from(v).unwrap());
        };

        set(&mut palette_container, 0, 0); // 0 bits
        assert_eq!(palette_container.bits_per_entry, 0);

        set(&mut palette_container, 1, 1); // 1 bit
        assert_eq!(palette_container.bits_per_entry, 1);

        set(&mut palette_container, 2, 2); // 2 bits
        assert_eq!(palette_container.bits_per_entry, 2);
        set(&mut palette_container, 3, 3);

        set(&mut palette_container, 4, 4); // 3 bits
        assert_eq!(palette_container.bits_per_entry, 3);
        set(&mut palette_container, 5, 5);
        set(&mut palette_container, 6, 6);
        set(&mut palette_container, 7, 7);

        set(&mut palette_container, 8, 8); // 4 bits
        assert_eq!(palette_container.bits_per_entry, 4);
        set(&mut palette_container, 9, 9);
        set(&mut palette_container, 10, 10);
        set(&mut palette_container, 11, 11);
        set(&mut palette_container, 12, 12);
        set(&mut palette_container, 13, 13);
        set(&mut palette_container, 14, 14);
        set(&mut palette_container, 15, 15);
        assert_eq!(palette_container.bits_per_entry, 4);

        set(&mut palette_container, 16, 16); // 5 bits
        assert_eq!(palette_container.bits_per_entry, 5);
    }

    #[test]
    fn test_coords_from_index() {
        let palette_container = PalettedContainer::new(PalettedContainerKind::BlockStates);

        for x in 0..15 {
            for y in 0..15 {
                for z in 0..15 {
                    assert_eq!(
                        palette_container
                            .coords_from_index(palette_container.index_from_coords(x, y, z)),
                        (x, y, z)
                    );
                }
            }
        }
    }
}
