use std::{
    collections::{HashMap, hash_map::Entry},
    fmt,
    fmt::Debug,
    io,
    io::{Cursor, Write},
    sync::{Arc, Weak},
};

use azalea_block::{
    block_state::{BlockState, BlockStateIntegerRepr},
    fluid_state::FluidState,
};
use azalea_buf::{AzaleaRead, AzaleaWrite, BufReadError};
use azalea_core::position::{
    BlockPos, ChunkBiomePos, ChunkBlockPos, ChunkPos, ChunkSectionBiomePos, ChunkSectionBlockPos,
};
use azalea_registry::Biome;
use nohash_hasher::IntMap;
use parking_lot::RwLock;
use tracing::{debug, trace, warn};

use crate::{
    heightmap::{Heightmap, HeightmapKind},
    palette::PalettedContainer,
};

const SECTION_HEIGHT: u32 = 16;

/// An efficient storage of chunks for a client that has a limited render
/// distance.
///
/// This has support for using a shared [`ChunkStorage`].
pub struct PartialChunkStorage {
    /// The center of the view, i.e. the chunk the player is currently in.
    view_center: ChunkPos,
    pub(crate) chunk_radius: u32,
    view_range: u32,
    // chunks is a list of size chunk_radius * chunk_radius
    chunks: Box<[Option<Arc<RwLock<Chunk>>>]>,
}

/// A storage for chunks where they're only stored weakly, so if they're not
/// actively being used somewhere else they'll be forgotten.
///
/// This is used for shared worlds.
///
/// This is relatively cheap to clone since it's just an `IntMap` with `Weak`
/// pointers.
#[derive(Debug, Clone)]
pub struct ChunkStorage {
    pub height: u32,
    pub min_y: i32,
    pub map: IntMap<ChunkPos, Weak<RwLock<Chunk>>>,
}

/// A single chunk in a world (16*?*16 blocks).
///
/// This only contains blocks and biomes. You can derive the height of the chunk
/// from the number of sections, but you need a [`ChunkStorage`] to get the
/// minimum Y coordinate.
#[derive(Debug)]
pub struct Chunk {
    pub sections: Box<[Section]>,
    /// Heightmaps are used for identifying the surface blocks in a chunk.
    /// Usually for clients only `WorldSurface` and `MotionBlocking` are
    /// present.
    pub heightmaps: HashMap<HeightmapKind, Heightmap>,
}

/// A section of a chunk, i.e. a 16*16*16 block area.
#[derive(Clone, Debug, Default)]
pub struct Section {
    pub block_count: u16,
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

impl PartialChunkStorage {
    pub fn new(chunk_radius: u32) -> Self {
        let view_range = chunk_radius * 2 + 1;
        PartialChunkStorage {
            view_center: ChunkPos::new(0, 0),
            chunk_radius,
            view_range,
            chunks: vec![None; (view_range * view_range) as usize].into(),
        }
    }

    /// Update the chunk to center the view on.
    ///
    /// This should be called when the client receives a `SetChunkCacheCenter`
    /// packet.
    pub fn update_view_center(&mut self, view_center: ChunkPos) {
        // this code block makes it force unload the chunks that are out of range after
        // updating the view center. it's usually fine without it but the commented code
        // is there in case you want to temporarily uncomment to test something

        // ```
        // for index in 0..self.chunks.len() {
        //     let chunk_pos = self.chunk_pos_from_index(index);
        //     if !in_range_for_view_center_and_radius(&chunk_pos, view_center, self.chunk_radius) {
        //         self.chunks[index] = None;
        //     }
        // }
        // ```

        self.view_center = view_center;
    }

    /// Get the center of the view. This is usually the chunk that the player is
    /// in.
    pub fn view_center(&self) -> ChunkPos {
        self.view_center
    }

    pub fn view_range(&self) -> u32 {
        self.view_range
    }

    pub fn index_from_chunk_pos(&self, chunk_pos: &ChunkPos) -> usize {
        let view_range = self.view_range as i32;

        let x = i32::rem_euclid(chunk_pos.x, view_range) * view_range;
        let z = i32::rem_euclid(chunk_pos.z, view_range);
        (x + z) as usize
    }

    pub fn chunk_pos_from_index(&self, index: usize) -> ChunkPos {
        let view_range = self.view_range as i32;

        // find the base from the view center
        let base_x = self.view_center.x.div_euclid(view_range) * view_range;
        let base_z = self.view_center.z.div_euclid(view_range) * view_range;

        // add the offset from the base
        let offset_x = index as i32 / view_range;
        let offset_z = index as i32 % view_range;

        ChunkPos::new(base_x + offset_x, base_z + offset_z)
    }

    pub fn in_range(&self, chunk_pos: &ChunkPos) -> bool {
        in_range_for_view_center_and_radius(chunk_pos, self.view_center, self.chunk_radius)
    }

    pub fn set_block_state(
        &self,
        pos: BlockPos,
        state: BlockState,
        chunk_storage: &ChunkStorage,
    ) -> Option<BlockState> {
        if pos.y < chunk_storage.min_y
            || pos.y >= (chunk_storage.min_y + chunk_storage.height as i32)
        {
            return None;
        }
        let chunk_pos = ChunkPos::from(pos);
        let chunk_lock = chunk_storage.get(&chunk_pos)?;
        let mut chunk = chunk_lock.write();
        Some(chunk.get_and_set_block_state(&ChunkBlockPos::from(pos), state, chunk_storage.min_y))
    }

    pub fn replace_with_packet_data(
        &mut self,
        pos: &ChunkPos,
        data: &mut Cursor<&[u8]>,
        heightmaps: &[(HeightmapKind, Box<[u64]>)],
        chunk_storage: &mut ChunkStorage,
    ) -> Result<(), BufReadError> {
        debug!("Replacing chunk at {:?}", pos);
        if !self.in_range(pos) {
            warn!("Ignoring chunk since it's not in the view range: {pos:?}");
            return Ok(());
        }

        let chunk = Chunk::read_with_dimension_height(
            data,
            chunk_storage.height,
            chunk_storage.min_y,
            heightmaps,
        )?;

        self.set(pos, Some(chunk), chunk_storage);
        trace!("Loaded chunk {pos:?}");

        Ok(())
    }

    /// Get a [`Chunk`] within render distance, or `None` if it's not loaded.
    /// Use [`ChunkStorage::get`] to get a chunk from the shared storage.
    pub fn limited_get(&self, pos: &ChunkPos) -> Option<&Arc<RwLock<Chunk>>> {
        if !self.in_range(pos) {
            warn!(
                "Chunk at {:?} is not in the render distance (center: {:?}, {} chunks)",
                pos, self.view_center, self.chunk_radius,
            );
            return None;
        }

        let index = self.index_from_chunk_pos(pos);
        self.chunks[index].as_ref()
    }
    /// Get a mutable reference to a [`Chunk`] within render distance, or
    /// `None` if it's not loaded.
    ///
    /// Use [`ChunkStorage::get`] to get a chunk from the shared storage.
    pub fn limited_get_mut(&mut self, pos: &ChunkPos) -> Option<&mut Option<Arc<RwLock<Chunk>>>> {
        if !self.in_range(pos) {
            return None;
        }

        let index = self.index_from_chunk_pos(pos);

        Some(&mut self.chunks[index])
    }

    /// Set a chunk in the shared storage and reference it from the limited
    /// storage.
    ///
    /// Use [`Self::limited_set`] if you already have an `Arc<RwLock<Chunk>>`.
    ///
    /// # Panics
    /// If the chunk is not in the render distance.
    pub fn set(&mut self, pos: &ChunkPos, chunk: Option<Chunk>, chunk_storage: &mut ChunkStorage) {
        let new_chunk;

        // add the chunk to the shared storage
        if let Some(chunk) = chunk {
            match chunk_storage.map.entry(*pos) {
                Entry::Occupied(mut e) => {
                    if let Some(old_chunk) = e.get_mut().upgrade() {
                        *old_chunk.write() = chunk;
                        new_chunk = Some(old_chunk);
                    } else {
                        let chunk_lock = Arc::new(RwLock::new(chunk));
                        e.insert(Arc::downgrade(&chunk_lock));
                        new_chunk = Some(chunk_lock);
                    }
                }
                Entry::Vacant(e) => {
                    let chunk_lock = Arc::new(RwLock::new(chunk));
                    e.insert(Arc::downgrade(&chunk_lock));
                    new_chunk = Some(chunk_lock);
                }
            }
        } else {
            // don't remove it from the shared storage, since it'll be removed
            // automatically if this was the last reference

            new_chunk = None;
        }

        self.limited_set(pos, new_chunk);
    }

    /// Set a chunk in our limited storage, useful if your chunk is already
    /// referenced somewhere else and you want to make it also be referenced by
    /// this storage.
    ///
    /// Use [`Self::set`] if you don't already have an `Arc<RwLock<Chunk>>`.
    ///
    /// # Panics
    /// If the chunk is not in the render distance.
    pub fn limited_set(&mut self, pos: &ChunkPos, chunk: Option<Arc<RwLock<Chunk>>>) {
        if let Some(chunk_mut) = self.limited_get_mut(pos) {
            *chunk_mut = chunk;
        }
    }

    /// Get an iterator over all the chunks in the storage.
    pub fn chunks(&self) -> impl Iterator<Item = &Option<Arc<RwLock<Chunk>>>> {
        self.chunks.iter()
    }
}
impl ChunkStorage {
    pub fn new(height: u32, min_y: i32) -> Self {
        ChunkStorage {
            height,
            min_y,
            map: IntMap::default(),
        }
    }

    pub fn get(&self, pos: &ChunkPos) -> Option<Arc<RwLock<Chunk>>> {
        self.map.get(pos).and_then(|chunk| chunk.upgrade())
    }

    pub fn get_block_state(&self, pos: BlockPos) -> Option<BlockState> {
        let chunk_pos = ChunkPos::from(pos);
        let chunk = self.get(&chunk_pos)?;
        let chunk = chunk.read();
        chunk.get_block_state(&ChunkBlockPos::from(pos), self.min_y)
    }

    pub fn get_fluid_state(&self, pos: BlockPos) -> Option<FluidState> {
        let block_state = self.get_block_state(pos)?;
        Some(FluidState::from(block_state))
    }

    pub fn get_biome(&self, pos: BlockPos) -> Option<Biome> {
        let chunk_pos = ChunkPos::from(pos);
        let chunk = self.get(&chunk_pos)?;
        let chunk = chunk.read();
        chunk.get_biome(ChunkBiomePos::from(pos), self.min_y)
    }

    pub fn set_block_state(&self, pos: BlockPos, state: BlockState) -> Option<BlockState> {
        if pos.y < self.min_y || pos.y >= (self.min_y + self.height as i32) {
            return None;
        }
        let chunk_pos = ChunkPos::from(pos);
        let chunk = self.get(&chunk_pos)?;
        let mut chunk = chunk.write();
        Some(chunk.get_and_set_block_state(&ChunkBlockPos::from(pos), state, self.min_y))
    }
}

pub fn in_range_for_view_center_and_radius(
    chunk_pos: &ChunkPos,
    view_center: ChunkPos,
    chunk_radius: u32,
) -> bool {
    (chunk_pos.x - view_center.x).unsigned_abs() <= chunk_radius
        && (chunk_pos.z - view_center.z).unsigned_abs() <= chunk_radius
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
            let data: Box<[u64]> = data.clone();
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
        section.set_block_state(chunk_section_pos, state);

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

impl AzaleaWrite for Chunk {
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        for section in &self.sections {
            section.azalea_write(buf)?;
        }
        Ok(())
    }
}

impl Debug for PartialChunkStorage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PartialChunkStorage")
            .field("view_center", &self.view_center)
            .field("chunk_radius", &self.chunk_radius)
            .field("view_range", &self.view_range)
            // .field("chunks", &self.chunks)
            .field("chunks", &format_args!("{} items", self.chunks.len()))
            .finish()
    }
}

impl AzaleaRead for Section {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let block_count = u16::azalea_read(buf)?;

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
            states,
            biomes,
        })
    }
}

impl AzaleaWrite for Section {
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        self.block_count.azalea_write(buf)?;
        self.states.azalea_write(buf)?;
        self.biomes.azalea_write(buf)?;
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
        self.states.get_and_set(pos, state)
    }
    pub fn set_block_state(&mut self, pos: ChunkSectionBlockPos, state: BlockState) {
        self.states.set(pos, state);
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

impl Default for PartialChunkStorage {
    fn default() -> Self {
        Self::new(8)
    }
}
impl Default for ChunkStorage {
    fn default() -> Self {
        Self::new(384, -64)
    }
}

/// Get the index of where a section is in a chunk based on its y coordinate
/// and the minimum y coordinate of the world.
#[inline]
pub fn section_index(y: i32, min_y: i32) -> u32 {
    if y < min_y {
        #[cfg(debug_assertions)]
        warn!("y ({y}) must be at least {min_y}");
        #[cfg(not(debug_assertions))]
        trace!("y ({y}) must be at least {min_y}")
    };
    let min_section_index = min_y >> 4;
    ((y >> 4) - min_section_index) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_out_of_bounds_y() {
        let mut chunk_storage = ChunkStorage::default();
        let mut partial_chunk_storage = PartialChunkStorage::default();
        partial_chunk_storage.set(
            &ChunkPos { x: 0, z: 0 },
            Some(Chunk::default()),
            &mut chunk_storage,
        );
        assert!(
            chunk_storage
                .get_block_state(BlockPos { x: 0, y: 319, z: 0 })
                .is_some()
        );
        assert!(
            chunk_storage
                .get_block_state(BlockPos { x: 0, y: 320, z: 0 })
                .is_none()
        );
        assert!(
            chunk_storage
                .get_block_state(BlockPos { x: 0, y: 338, z: 0 })
                .is_none()
        );
        assert!(
            chunk_storage
                .get_block_state(BlockPos { x: 0, y: -64, z: 0 })
                .is_some()
        );
        assert!(
            chunk_storage
                .get_block_state(BlockPos { x: 0, y: -65, z: 0 })
                .is_none()
        );
    }

    #[test]
    fn test_chunk_pos_from_index() {
        let mut partial_chunk_storage = PartialChunkStorage::new(5);
        partial_chunk_storage.update_view_center(ChunkPos::new(0, -1));
        assert_eq!(
            partial_chunk_storage.chunk_pos_from_index(
                partial_chunk_storage.index_from_chunk_pos(&ChunkPos::new(2, -1))
            ),
            ChunkPos::new(2, -1),
        );
    }
}
