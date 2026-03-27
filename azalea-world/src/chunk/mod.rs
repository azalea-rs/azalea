pub mod partial;
pub mod storage;

use std::{
    collections::HashMap,
    fmt::Debug,
    io,
    io::{Cursor, Write},
};

use azalea_block::block_state::{BlockState, BlockStateIntegerRepr};
use azalea_buf::{AzBuf, BufReadError};
use azalea_core::{
    heightmap_kind::HeightmapKind,
    position::{ChunkBiomePos, ChunkBlockPos, ChunkSectionBiomePos, ChunkSectionBlockPos},
};
use azalea_registry::data::Biome;
use tracing::warn;

use crate::{heightmap::Heightmap, palette::PalettedContainer};

const SECTION_HEIGHT: u32 = 16;

/// A single chunk in a world (16*?*16 blocks).
///
/// This only contains blocks and biomes. You can derive the height of the chunk
/// from the number of sections, but you need a [`ChunkStorage`] to get the
/// minimum Y coordinate.
///
/// [`ChunkStorage`]: crate::ChunkStorage
#[derive(Debug)]
pub struct Chunk {
    pub sections: Box<[Section]>,
    /// Heightmaps are used for identifying the surface blocks in a chunk.
    /// Usually for clients only `WorldSurface` and `MotionBlocking` are
    /// present.
    pub heightmaps: HashMap<HeightmapKind, Heightmap>,
}

/// A section of a chunk, i.e. a 16*16*16 block area.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Section {
    /// The number of non-empty blocks in the section, which is initialized
    /// based on a value sent to us by the server.
    ///
    /// This may be updated every time [`Self::get_and_set_block_state`] is
    /// called.
    pub block_count: u16,
    /// Similar to [`Self::block_count`], but for fluids.
    ///
    /// Unlike [`Self::block_count`], this is currently not updated by Azalea.
    pub fluid_count: u16,
    pub states: PalettedContainer<BlockState>,
    pub biomes: PalettedContainer<Biome>,
}

/// Get the actual stored view distance for the selected view distance.
///
/// For some reason, Minecraft stores an extra 3 chunks.
pub fn calculate_chunk_storage_range(view_distance: u32) -> u32 {
    u32::max(view_distance, 2) + 3
}

impl Default for Chunk {
    fn default() -> Self {
        Chunk {
            sections: vec![Section::default(); (384 / 16) as usize].into(),
            heightmaps: HashMap::new(),
        }
    }
}

impl Chunk {
    pub fn read_with_dimension_height(
        buf: &mut Cursor<&[u8]>,
        dimension_height: u32,
        min_y: i32,
        heightmaps_data: &[(HeightmapKind, Box<[u64]>)],
    ) -> Result<Self, BufReadError> {
        let section_count = dimension_height / SECTION_HEIGHT;
        let mut sections = Vec::with_capacity(section_count as usize);
        for _ in 0..section_count {
            let section = Section::azalea_read(buf)?;
            sections.push(section);
        }
        let sections = sections.into_boxed_slice();

        let mut heightmaps = HashMap::new();
        for (kind, data) in heightmaps_data {
            let data = data.clone();
            let heightmap = Heightmap::new(*kind, dimension_height, min_y, data);
            heightmaps.insert(*kind, heightmap);
        }

        Ok(Chunk {
            sections,
            heightmaps,
        })
    }

    pub fn get_block_state(&self, pos: &ChunkBlockPos, min_y: i32) -> Option<BlockState> {
        get_block_state_from_sections(&self.sections, pos, min_y)
    }

    #[must_use = "Use Chunk::set_block_state instead if you don't need the previous state"]
    pub fn get_and_set_block_state(
        &mut self,
        pos: &ChunkBlockPos,
        state: BlockState,
        min_y: i32,
    ) -> BlockState {
        let section_index = section_index(pos.y, min_y);
        let Some(section) = self.sections.get_mut(section_index as usize) else {
            warn!(
                "Tried to get and set block state {state:?} at out-of-bounds relative chunk position {pos:?}",
            );
            return BlockState::AIR;
        };
        let chunk_section_pos = ChunkSectionBlockPos::from(pos);
        let previous_state = section.get_and_set_block_state(chunk_section_pos, state);

        for heightmap in self.heightmaps.values_mut() {
            heightmap.update(pos, state, &self.sections);
        }

        previous_state
    }

    pub fn set_block_state(&mut self, pos: &ChunkBlockPos, state: BlockState, min_y: i32) {
        let section_index = section_index(pos.y, min_y);
        let Some(section) = self.sections.get_mut(section_index as usize) else {
            warn!(
                "Tried to set block state {state:?} at out-of-bounds relative chunk position {pos:?}",
            );
            return;
        };
        let chunk_section_pos = ChunkSectionBlockPos::from(pos);
        section.get_and_set_block_state(chunk_section_pos, state);

        for heightmap in self.heightmaps.values_mut() {
            heightmap.update(pos, state, &self.sections);
        }
    }

    /// Get the biome at the given position, or `None` if it's out of bounds.
    pub fn get_biome(&self, pos: ChunkBiomePos, min_y: i32) -> Option<Biome> {
        if pos.y < min_y {
            // y position is out of bounds
            return None;
        }
        let section_index = section_index(pos.y, min_y);
        let Some(section) = self.sections.get(section_index as usize) else {
            warn!("Tried to get biome at out-of-bounds relative chunk position {pos:?}",);
            return None;
        };
        let chunk_section_pos = ChunkSectionBiomePos::from(pos);
        Some(section.get_biome(chunk_section_pos))
    }
}

/// Get the block state at the given position from a list of sections. Returns
/// `None` if the position is out of bounds.
#[inline]
pub fn get_block_state_from_sections(
    sections: &[Section],
    pos: &ChunkBlockPos,
    min_y: i32,
) -> Option<BlockState> {
    if pos.y < min_y {
        // y position is out of bounds
        return None;
    }
    let section_index = section_index(pos.y, min_y) as usize;
    if section_index >= sections.len() {
        // y position is out of bounds
        return None;
    };
    let section = &sections[section_index];

    let chunk_section_pos = ChunkSectionBlockPos::from(pos);
    Some(section.get_block_state(chunk_section_pos))
}

impl AzBuf for Section {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let block_count = u16::azalea_read(buf)?;
        let fluid_count = u16::azalea_read(buf)?;

        // this is commented out because the vanilla server is wrong
        // TODO: ^ this comment was written ages ago. needs more investigation.
        // assert!(
        //     block_count <= 16 * 16 * 16,
        //     "A section has more blocks than what should be possible. This is a bug!"
        // );

        let states = PalettedContainer::<BlockState>::read(buf)?;

        for i in 0..states.storage.size() {
            if !BlockState::is_valid_state(states.storage.get(i) as BlockStateIntegerRepr) {
                return Err(BufReadError::Custom(format!(
                    "Invalid block state {} (index {i}) found in section.",
                    states.storage.get(i)
                )));
            }
        }

        let biomes = PalettedContainer::<Biome>::read(buf)?;
        Ok(Section {
            block_count,
            fluid_count,
            states,
            biomes,
        })
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        self.block_count.azalea_write(buf)?;
        self.fluid_count.azalea_write(buf)?;
        self.states.write(buf)?;
        self.biomes.write(buf)?;
        Ok(())
    }
}

impl Section {
    pub fn get_block_state(&self, pos: ChunkSectionBlockPos) -> BlockState {
        self.states.get(pos)
    }
    pub fn get_and_set_block_state(
        &mut self,
        pos: ChunkSectionBlockPos,
        state: BlockState,
    ) -> BlockState {
        let previous_state = self.states.get_and_set(pos, state);

        if previous_state.is_air() && !state.is_air() {
            self.block_count += 1;
        } else if !previous_state.is_air() && state.is_air() {
            self.block_count -= 1;
        }

        previous_state
    }

    pub fn get_biome(&self, pos: ChunkSectionBiomePos) -> Biome {
        self.biomes.get(pos)
    }
    pub fn set_biome(&mut self, pos: ChunkSectionBiomePos, biome: Biome) {
        self.biomes.set(pos, biome);
    }
    pub fn get_and_set_biome(&mut self, pos: ChunkSectionBiomePos, biome: Biome) -> Biome {
        self.biomes.get_and_set(pos, biome)
    }
}

/// Get the index of where a section is in a chunk based on its y coordinate
/// and the minimum y coordinate of the world.
#[inline]
pub fn section_index(y: i32, min_y: i32) -> u32 {
    if y < min_y {
        #[cfg(debug_assertions)]
        tracing::warn!("y ({y}) must be at least {min_y}");
        #[cfg(not(debug_assertions))]
        tracing::trace!("y ({y}) must be at least {min_y}")
    };
    let min_section_index = min_y >> 4;
    ((y >> 4) - min_section_index) as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::palette::SectionPos;

    #[test]
    fn test_section_index() {
        assert_eq!(section_index(0, 0), 0);
        assert_eq!(section_index(128, 0), 8);
        assert_eq!(section_index(127, 0), 7);
        assert_eq!(section_index(0, -64), 4);
        assert_eq!(section_index(-64, -64), 0);
        assert_eq!(section_index(-49, -64), 0);
        assert_eq!(section_index(-48, -64), 1);
        assert_eq!(section_index(128, -64), 12);
    }

    #[test]
    fn serialize_and_deserialize_section() {
        let mut states = PalettedContainer::new();

        states.set(
            SectionPos::new(1, 2, 3),
            BlockState::try_from(BlockState::MAX_STATE).unwrap(),
        );
        states.set(
            SectionPos::new(4, 5, 6),
            BlockState::try_from(BlockState::MAX_STATE).unwrap(),
        );
        let biomes = PalettedContainer::new();
        let section = Section {
            block_count: 2,
            fluid_count: 0,
            states,
            biomes,
        };

        let mut buf = Vec::new();
        section.azalea_write(&mut buf).unwrap();

        let mut cur = Cursor::new(buf.as_slice());
        let deserialized_section = Section::azalea_read(&mut cur).unwrap();
        assert_eq!(cur.position(), buf.len() as u64);

        assert_eq!(section, deserialized_section);
    }
}
