use crate::palette::PalettedContainer;
use crate::palette::PalettedContainerType;
use crate::Dimension;
use azalea_block::BlockState;
use azalea_buf::BufReadError;
use azalea_buf::{McBufReadable, McBufWritable};
use azalea_core::floor_mod;
use azalea_core::{BlockPos, ChunkBlockPos, ChunkPos, ChunkSectionBlockPos};
use std::fmt::Debug;
use std::io::Cursor;
use std::{
    io::Write,
    ops::{Index, IndexMut},
    sync::{Arc, Mutex},
};

const SECTION_HEIGHT: u32 = 16;

pub struct ChunkStorage {
    pub view_center: ChunkPos,
    chunk_radius: u32,
    view_range: u32,
    pub height: u32,
    pub min_y: i32,
    // chunks is a list of size chunk_radius * chunk_radius
    chunks: Vec<Option<Arc<Mutex<Chunk>>>>,
}

#[derive(Debug)]
pub struct Chunk {
    pub sections: Vec<Section>,
}

#[derive(Clone, Debug)]
pub struct Section {
    pub block_count: u16,
    pub states: PalettedContainer,
    pub biomes: PalettedContainer,
}

impl Default for Section {
    fn default() -> Self {
        Section {
            block_count: 0,
            states: PalettedContainer::new(&PalettedContainerType::BlockStates).unwrap(),
            biomes: PalettedContainer::new(&PalettedContainerType::Biomes).unwrap(),
        }
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Chunk {
            sections: vec![Section::default(); (384 / 16) as usize],
        }
    }
}

impl ChunkStorage {
    pub fn new(chunk_radius: u32, height: u32, min_y: i32) -> Self {
        let view_range = chunk_radius * 2 + 1;
        ChunkStorage {
            view_center: ChunkPos::new(0, 0),
            chunk_radius,
            view_range,
            height,
            min_y,
            chunks: vec![None; (view_range * view_range) as usize],
        }
    }

    fn get_index(&self, chunk_pos: &ChunkPos) -> usize {
        (floor_mod(chunk_pos.x, self.view_range) * self.view_range
            + floor_mod(chunk_pos.z, self.view_range)) as usize
    }

    pub fn in_range(&self, chunk_pos: &ChunkPos) -> bool {
        (chunk_pos.x - self.view_center.x).unsigned_abs() <= self.chunk_radius
            && (chunk_pos.z - self.view_center.z).unsigned_abs() <= self.chunk_radius
    }

    pub fn get_block_state(&self, pos: &BlockPos) -> Option<BlockState> {
        let chunk_pos = ChunkPos::from(pos);
        let chunk = self[&chunk_pos].as_ref()?;
        let chunk = chunk.lock().unwrap();
        chunk.get(&ChunkBlockPos::from(pos), self.min_y)
    }

    pub fn set_block_state(&self, pos: &BlockPos, state: BlockState) -> Option<BlockState> {
        if pos.y < self.min_y || pos.y >= (self.min_y + self.height as i32) {
            return None;
        }
        let chunk_pos = ChunkPos::from(pos);
        let chunk = self[&chunk_pos].as_ref()?;
        let mut chunk = chunk.lock().unwrap();
        Some(chunk.get_and_set(&ChunkBlockPos::from(pos), state, self.min_y))
    }

    pub fn replace_with_packet_data(
        &mut self,
        pos: &ChunkPos,
        data: &mut Cursor<&[u8]>,
    ) -> Result<(), BufReadError> {
        if !self.in_range(pos) {
            log::trace!(
                "Ignoring chunk since it's not in the view range: {}, {}",
                pos.x,
                pos.z
            );
            return Ok(());
        }

        let chunk = Arc::new(Mutex::new(Chunk::read_with_dimension_height(
            data,
            self.height,
        )?));

        log::trace!("Loaded chunk {:?}", pos);
        self[pos] = Some(chunk);

        Ok(())
    }
}

impl Index<&ChunkPos> for ChunkStorage {
    type Output = Option<Arc<Mutex<Chunk>>>;

    fn index(&self, pos: &ChunkPos) -> &Self::Output {
        &self.chunks[self.get_index(pos)]
    }
}
impl IndexMut<&ChunkPos> for ChunkStorage {
    fn index_mut<'a>(&'a mut self, pos: &ChunkPos) -> &'a mut Self::Output {
        let index = self.get_index(pos);
        &mut self.chunks[index]
    }
}

impl Chunk {
    pub fn read_with_dimension(
        buf: &mut Cursor<&[u8]>,
        data: &Dimension,
    ) -> Result<Self, BufReadError> {
        Self::read_with_dimension_height(buf, data.height())
    }

    pub fn read_with_dimension_height(
        buf: &mut Cursor<&[u8]>,
        dimension_height: u32,
    ) -> Result<Self, BufReadError> {
        let section_count = dimension_height / SECTION_HEIGHT;
        let mut sections = Vec::with_capacity(section_count as usize);
        for _ in 0..section_count {
            let section = Section::read_from(buf)?;
            sections.push(section);
        }
        Ok(Chunk { sections })
    }

    pub fn section_index(&self, y: i32, min_y: i32) -> u32 {
        assert!(y >= min_y, "y ({y}) must be at least {min_y}");
        let min_section_index = min_y.div_floor(16);
        (y.div_floor(16) - min_section_index) as u32
    }

    pub fn get(&self, pos: &ChunkBlockPos, min_y: i32) -> Option<BlockState> {
        let section_index = self.section_index(pos.y, min_y) as usize;
        if section_index >= self.sections.len() {
            // y position is out of bounds
            return None;
        };
        // TODO: make sure the section exists
        let section = &self.sections[section_index];
        let chunk_section_pos = ChunkSectionBlockPos::from(pos);
        Some(section.get(chunk_section_pos))
    }

    pub fn get_and_set(
        &mut self,
        pos: &ChunkBlockPos,
        state: BlockState,
        min_y: i32,
    ) -> BlockState {
        let section_index = self.section_index(pos.y, min_y);
        // TODO: make sure the section exists
        let section = &mut self.sections[section_index as usize];
        let chunk_section_pos = ChunkSectionBlockPos::from(pos);
        section.get_and_set(chunk_section_pos, state)
    }

    pub fn set(&mut self, pos: &ChunkBlockPos, state: BlockState, min_y: i32) {
        let section_index = self.section_index(pos.y, min_y);
        // TODO: make sure the section exists
        let section = &mut self.sections[section_index as usize];
        let chunk_section_pos = ChunkSectionBlockPos::from(pos);
        section.set(chunk_section_pos, state)
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

impl Debug for ChunkStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChunkStorage")
            .field("view_center", &self.view_center)
            .field("chunk_radius", &self.chunk_radius)
            .field("view_range", &self.view_range)
            .field("height", &self.height)
            .field("min_y", &self.min_y)
            // .field("chunks", &self.chunks)
            .field("chunks", &format_args!("{} items", self.chunks.len()))
            .finish()
    }
}

impl McBufReadable for Section {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let block_count = u16::read_from(buf)?;

        // this is commented out because the vanilla server is wrong
        // assert!(
        //     block_count <= 16 * 16 * 16,
        //     "A section has more blocks than what should be possible. This is a bug!"
        // );

        let states = PalettedContainer::read_with_type(buf, &PalettedContainerType::BlockStates)?;

        for i in 0..states.storage.size() {
            if !BlockState::is_valid_state(states.storage.get(i) as u32) {
                return Err(BufReadError::Custom(format!(
                    "Invalid block state {} (index {}) found in section.",
                    states.storage.get(i),
                    i
                )));
            }
        }

        let biomes = PalettedContainer::read_with_type(buf, &PalettedContainerType::Biomes)?;
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

impl Section {
    fn get(&self, pos: ChunkSectionBlockPos) -> BlockState {
        // TODO: use the unsafe method and do the check earlier
        let state = self
            .states
            .get(pos.x as usize, pos.y as usize, pos.z as usize);
        // if there's an unknown block assume it's air
        BlockState::try_from(state).unwrap_or(BlockState::Air)
    }

    fn get_and_set(&mut self, pos: ChunkSectionBlockPos, state: BlockState) -> BlockState {
        let previous_state =
            self.states
                .get_and_set(pos.x as usize, pos.y as usize, pos.z as usize, state as u32);
        // if there's an unknown block assume it's air
        BlockState::try_from(previous_state).unwrap_or(BlockState::Air)
    }

    fn set(&mut self, pos: ChunkSectionBlockPos, state: BlockState) {
        self.states
            .set(pos.x as usize, pos.y as usize, pos.z as usize, state as u32);
    }
}

impl Default for ChunkStorage {
    fn default() -> Self {
        Self::new(8, 384, -64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_index() {
        let chunk = Chunk::default();
        assert_eq!(chunk.section_index(0, 0), 0);
        assert_eq!(chunk.section_index(128, 0), 8);
        assert_eq!(chunk.section_index(127, 0), 7);
        assert_eq!(chunk.section_index(0, -64), 4);
        assert_eq!(chunk.section_index(-64, -64), 0);
        assert_eq!(chunk.section_index(-49, -64), 0);
        assert_eq!(chunk.section_index(-48, -64), 1);
        assert_eq!(chunk.section_index(128, -64), 12);
    }

    #[test]
    fn test_out_of_bounds_y() {
        let mut chunk_storage = ChunkStorage::default();
        chunk_storage[&ChunkPos { x: 0, z: 0 }] = Some(Arc::new(Mutex::new(Chunk::default())));
        assert!(chunk_storage
            .get_block_state(&BlockPos { x: 0, y: 319, z: 0 })
            .is_some());
        assert!(chunk_storage
            .get_block_state(&BlockPos { x: 0, y: 320, z: 0 })
            .is_none());
        assert!(chunk_storage
            .get_block_state(&BlockPos { x: 0, y: 338, z: 0 })
            .is_none());
        assert!(chunk_storage
            .get_block_state(&BlockPos { x: 0, y: -64, z: 0 })
            .is_some());
        assert!(chunk_storage
            .get_block_state(&BlockPos { x: 0, y: -65, z: 0 })
            .is_none());
    }
}
