use crate::palette::PalettedContainer;
use crate::palette::PalettedContainerType;
use crate::World;
use azalea_block::BlockState;
use azalea_buf::BufReadError;
use azalea_buf::{McBufReadable, McBufWritable};
use azalea_core::{BlockPos, ChunkBlockPos, ChunkPos, ChunkSectionBlockPos};
use log::debug;
use log::trace;
use log::warn;
use parking_lot::Mutex;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::Cursor;
use std::sync::Weak;
use std::{io::Write, sync::Arc};

const SECTION_HEIGHT: u32 = 16;

/// An efficient storage of chunks for a client that has a limited render
/// distance. This has support for using a shared [`WeakChunkStorage`]. If you
/// have an infinite render distance (like a server), you should use
/// [`ChunkStorage`] instead.
pub struct PartialChunkStorage {
    /// Chunk storage that can be shared by clients.
    shared: Arc<RwLock<WeakChunkStorage>>,

    pub view_center: ChunkPos,
    chunk_radius: u32,
    view_range: u32,
    // chunks is a list of size chunk_radius * chunk_radius
    chunks: Vec<Option<Arc<Mutex<Chunk>>>>,
}

/// A storage for chunks where they're only stored weakly, so if they're not
/// actively being used somewhere else they'll be forgotten. This is used for
/// shared worlds.
pub struct WeakChunkStorage {
    pub height: u32,
    pub min_y: i32,
    pub chunks: HashMap<ChunkPos, Weak<Mutex<Chunk>>>,
}

/// A storage of potentially infinite chunks in a world. Chunks are stored as
/// an `Arc<Mutex>` so they can be shared across threads.
pub struct ChunkStorage {
    pub height: u32,
    pub min_y: i32,
    pub chunks: HashMap<ChunkPos, Arc<Mutex<Chunk>>>,
}

/// A single chunk in a world (16*?*16 blocks). This only contains the blocks and biomes. You
/// can derive the height of the chunk from the number of sections, but you
/// need a [`ChunkStorage`] to get the minimum Y coordinate.
#[derive(Debug)]
pub struct Chunk {
    pub sections: Vec<Section>,
}

/// A section of a chunk, i.e. a 16*16*16 block area.
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

impl PartialChunkStorage {
    pub fn new(chunk_radius: u32, shared: Arc<RwLock<WeakChunkStorage>>) -> Self {
        let view_range = chunk_radius * 2 + 1;
        PartialChunkStorage {
            shared,
            view_center: ChunkPos::new(0, 0),
            chunk_radius,
            view_range,
            chunks: vec![None; (view_range * view_range) as usize],
        }
    }

    pub fn min_y(&self) -> i32 {
        self.shared.read().min_y
    }
    pub fn height(&self) -> u32 {
        self.shared.read().height
    }

    fn get_index(&self, chunk_pos: &ChunkPos) -> usize {
        (i32::rem_euclid(chunk_pos.x, self.view_range as i32) * (self.view_range as i32)
            + i32::rem_euclid(chunk_pos.z, self.view_range as i32)) as usize
    }

    pub fn in_range(&self, chunk_pos: &ChunkPos) -> bool {
        (chunk_pos.x - self.view_center.x).unsigned_abs() <= self.chunk_radius
            && (chunk_pos.z - self.view_center.z).unsigned_abs() <= self.chunk_radius
    }

    pub fn get_block_state(&self, pos: &BlockPos) -> Option<BlockState> {
        let chunk_pos = ChunkPos::from(pos);
        let chunk = self.get(&chunk_pos)?;
        let chunk = chunk.lock();
        chunk.get(&ChunkBlockPos::from(pos), self.min_y())
    }

    pub fn set_block_state(&self, pos: &BlockPos, state: BlockState) -> Option<BlockState> {
        if pos.y < self.min_y() || pos.y >= (self.min_y() + self.height() as i32) {
            return None;
        }
        let chunk_pos = ChunkPos::from(pos);
        let chunk = self.get(&chunk_pos)?;
        let mut chunk = chunk.lock();
        Some(chunk.get_and_set(&ChunkBlockPos::from(pos), state, self.min_y()))
    }

    pub fn replace_with_packet_data(
        &mut self,
        pos: &ChunkPos,
        data: &mut Cursor<&[u8]>,
    ) -> Result<(), BufReadError> {
        debug!("Replacing chunk at {:?}", pos);
        if !self.in_range(pos) {
            trace!(
                "Ignoring chunk since it's not in the view range: {}, {}",
                pos.x,
                pos.z
            );
            return Ok(());
        }

        let chunk = Arc::new(Mutex::new(Chunk::read_with_dimension_height(
            data,
            self.height(),
        )?));

        trace!("Loaded chunk {:?}", pos);
        self.set(pos, Some(chunk));

        Ok(())
    }

    /// Get a [`Chunk`] within render distance, or `None` if it's not loaded.
    /// Use [`PartialChunkStorage::get`] to get a chunk from the shared storage.
    pub fn limited_get(&self, pos: &ChunkPos) -> Option<&Arc<Mutex<Chunk>>> {
        if !self.in_range(pos) {
            warn!(
                "Chunk at {:?} is not in the render distance (center: {:?}, {} chunks)",
                pos, self.view_center, self.chunk_radius,
            );
            return None;
        }

        let index = self.get_index(pos);
        self.chunks[index].as_ref()
    }
    /// Get a mutable reference to a [`Chunk`] within render distance, or
    /// `None` if it's not loaded. Use [`PartialChunkStorage::get`] to get
    /// a chunk from the shared storage.
    pub fn limited_get_mut(&mut self, pos: &ChunkPos) -> Option<&mut Option<Arc<Mutex<Chunk>>>> {
        if !self.in_range(pos) {
            return None;
        }

        let index = self.get_index(pos);
        Some(&mut self.chunks[index])
    }

    /// Get a chunk,
    pub fn get(&self, pos: &ChunkPos) -> Option<Arc<Mutex<Chunk>>> {
        self.shared
            .read()
            .chunks
            .get(pos)
            .and_then(|chunk| chunk.upgrade())
    }

    /// Set a chunk in the shared storage and reference it from the limited
    /// storage.
    ///
    /// # Panics
    /// If the chunk is not in the render distance.
    pub fn set(&mut self, pos: &ChunkPos, chunk: Option<Arc<Mutex<Chunk>>>) {
        if let Some(chunk) = &chunk {
            self.shared
                .write()
                .chunks
                .insert(*pos, Arc::downgrade(chunk));
        } else {
            // don't remove it from the shared storage, since it'll be removed
            // automatically if this was the last reference
        }
        if let Some(chunk_mut) = self.limited_get_mut(pos) {
            *chunk_mut = chunk;
        }
    }
}
impl WeakChunkStorage {
    pub fn new(height: u32, min_y: i32) -> Self {
        WeakChunkStorage {
            height,
            min_y,
            chunks: HashMap::new(),
        }
    }
}

impl Chunk {
    pub fn read_with_dimension(
        buf: &mut Cursor<&[u8]>,
        data: &World,
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
        if pos.y < min_y {
            // y position is out of bounds
            return None;
        }
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

impl Debug for PartialChunkStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChunkStorage")
            .field("view_center", &self.view_center)
            .field("chunk_radius", &self.chunk_radius)
            .field("view_range", &self.view_range)
            .field("height", &self.height())
            .field("min_y", &self.min_y())
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

impl Default for PartialChunkStorage {
    fn default() -> Self {
        Self::new(8, Arc::new(RwLock::new(WeakChunkStorage::default())))
    }
}
impl Default for WeakChunkStorage {
    fn default() -> Self {
        Self::new(384, -64)
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
        let mut chunk_storage = PartialChunkStorage::default();
        chunk_storage.set(
            &ChunkPos { x: 0, z: 0 },
            Some(Arc::new(Mutex::new(Chunk::default()))),
        );
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
