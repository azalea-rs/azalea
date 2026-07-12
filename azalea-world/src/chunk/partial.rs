use std::{
    fmt::{self, Debug},
    io::Cursor,
    sync::{
        Arc,
        atomic::{AtomicI32, Ordering},
    },
};

use arc_swap::ArcSwapOption;
use azalea_block::BlockState;
use azalea_buf::BufReadError;
use azalea_core::{
    heightmap_kind::HeightmapKind,
    position::{BlockPos, ChunkBlockPos, ChunkPos},
};
use parking_lot::RwLock;
use tracing::{debug, trace, warn};

use crate::{Chunk, chunk::storage::ChunkStorage};

/// An efficient storage of chunks for a client that has a limited render
/// distance.
///
/// This has support for using a shared [`ChunkStorage`].
pub struct PartialChunkStorage {
    /// The center of the view, i.e. the chunk the player is currently in.
    view_center_x: AtomicI32,
    view_center_z: AtomicI32,
    view_range: u32,

    pub(crate) chunk_radius: u32,
    // chunks is a list of size chunk_radius * chunk_radius
    chunks: Box<[ArcSwapOption<RwLock<Chunk>>]>,
}

impl PartialChunkStorage {
    pub fn new(chunk_radius: u32) -> Self {
        let view_range = chunk_radius * 2 + 1;

        let storage_size = (view_range * view_range) as usize;
        let mut chunks = Vec::with_capacity(storage_size);
        chunks.resize_with(storage_size, ArcSwapOption::const_empty);

        PartialChunkStorage {
            chunk_radius,
            view_range,
            view_center_x: AtomicI32::new(0),
            view_center_z: AtomicI32::new(0),
            chunks: chunks.into_boxed_slice(),
        }
    }

    /// Update the chunk to center the view on.
    ///
    /// This should be called when the client receives a `SetChunkCacheCenter`
    /// packet.
    pub fn update_view_center(&self, view_center: ChunkPos) {
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

        self.view_center_x.store(view_center.x, Ordering::Relaxed);
        self.view_center_z.store(view_center.z, Ordering::Relaxed);
    }

    /// Get the center of the view. This is usually the chunk that the player is
    /// in.
    pub fn view_center(&self) -> ChunkPos {
        let x = self.view_center_x.load(Ordering::Relaxed);
        let z = self.view_center_z.load(Ordering::Relaxed);
        ChunkPos::new(x, z)
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
        let base = self.view_center();
        let base_x = base.x.div_euclid(view_range) * view_range;
        let base_z = base.z.div_euclid(view_range) * view_range;

        // add the offset from the base
        let offset_x = index as i32 / view_range;
        let offset_z = index as i32 % view_range;

        ChunkPos::new(base_x + offset_x, base_z + offset_z)
    }

    pub fn in_range(&self, chunk_pos: &ChunkPos) -> bool {
        in_range_for_view_center_and_radius(chunk_pos, self.view_center(), self.chunk_radius)
    }

    pub fn set_block_state(
        &self,
        pos: BlockPos,
        state: BlockState,
        chunk_storage: &ChunkStorage,
    ) -> Option<BlockState> {
        if pos.y < chunk_storage.min_y()
            || pos.y >= (chunk_storage.min_y() + chunk_storage.height() as i32)
        {
            return None;
        }
        let chunk_pos = ChunkPos::from(pos);
        let chunk_lock = chunk_storage.get(&chunk_pos)?;
        let mut chunk = chunk_lock.write();
        Some(chunk.get_and_set_block_state(&ChunkBlockPos::from(pos), state, chunk_storage.min_y()))
    }

    pub fn replace_with_packet_data(
        &self,
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
            chunk_storage.height(),
            chunk_storage.min_y(),
            heightmaps,
        )?;

        self.set(pos, Some(chunk), chunk_storage);
        trace!("Loaded chunk {pos:?}");

        Ok(())
    }

    /// Get a [`Chunk`] within render distance, or `None` if it's not loaded.
    /// Use [`ChunkStorageTrait::get`] to get a chunk from the shared storage.
    ///
    /// [`ChunkStorageTrait::get`]: crate::chunk::storage::ChunkStorageTrait::get
    pub fn limited_get(&self, pos: &ChunkPos) -> Option<Arc<RwLock<Chunk>>> {
        if !self.in_range(pos) {
            warn!(
                "Chunk at {:?} is not in the render distance (center: {:?}, {} chunks)",
                pos,
                self.view_center(),
                self.chunk_radius,
            );
            return None;
        }

        let guard = self.chunks.get(self.index_from_chunk_pos(pos))?;

        if guard.load().is_some() {
            guard.load_full()
        } else {
            None
        }
    }

    /// Set a chunk in the shared storage and reference it from the limited
    /// storage.
    ///
    /// Use [`Self::limited_set`] if you already have an `Arc<RwLock<Chunk>>`.
    ///
    /// # Panics
    /// If the chunk is not in the render distance.
    pub fn set(&self, pos: &ChunkPos, chunk: Option<Chunk>, chunk_storage: &mut ChunkStorage) {
        let new_chunk = chunk.map(|c| chunk_storage.upsert(*pos, c));
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
    pub fn limited_set(&self, pos: &ChunkPos, chunk: Option<Arc<RwLock<Chunk>>>) {
        if let Some(guard) = self.chunks.get(self.index_from_chunk_pos(pos)) {
            guard.store(chunk);
        }
    }

    /// Get an iterator over all the chunks in the storage.
    pub fn chunks(&self) -> impl Iterator<Item = &ArcSwapOption<RwLock<Chunk>>> {
        self.chunks.iter()
    }

    /// Clears the internal references to chunks and resets the view center.
    pub fn reset(&self) {
        self.update_view_center(ChunkPos::new(0, 0));
        self.chunks.iter().for_each(|chunk| chunk.store(None));
    }
}

impl Debug for PartialChunkStorage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PartialChunkStorage")
            .field("view_center", &self.view_center())
            .field("chunk_radius", &self.chunk_radius)
            .field("view_range", &self.view_range)
            // .field("chunks", &self.chunks)
            .field("chunks", &format_args!("{} items", self.chunks.len()))
            .finish()
    }
}

impl Default for PartialChunkStorage {
    fn default() -> Self {
        Self::new(8)
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

#[cfg(test)]
mod tests {
    use azalea_core::position::ChunkPos;

    use crate::chunk::partial::PartialChunkStorage;

    #[test]
    fn test_chunk_pos_from_index() {
        let partial_chunk_storage = PartialChunkStorage::new(5);
        partial_chunk_storage.update_view_center(ChunkPos::new(0, -1));
        assert_eq!(
            partial_chunk_storage.chunk_pos_from_index(
                partial_chunk_storage.index_from_chunk_pos(&ChunkPos::new(2, -1))
            ),
            ChunkPos::new(2, -1),
        );
    }
}
