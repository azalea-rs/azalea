use azalea_block::{BlockState, BlockStates};
use azalea_core::position::{BlockPos, ChunkPos};

use crate::{
    Chunk, World, chunk::storage::ChunkStorage, iterators::ChunkIterator, palette::Palette,
};

impl World {
    /// Find the coordinates of a block in the world.
    ///
    /// Note that this is sorted by `x+y+z` and not `x^2+y^2+z^2` for
    /// performance purposes.
    ///
    /// Also see [`Self::find_blocks`].
    ///
    /// ```
    /// # use azalea_registry::builtin::BlockKind;
    /// # fn example(position: azalea_core::position::Vec3, world: &azalea_world::World) {
    /// let pos = world.find_block(position, &BlockKind::Chest.into());
    /// # }
    /// ```
    pub fn find_block(
        &self,
        nearest_to: impl Into<BlockPos>,
        block_states: &BlockStates,
    ) -> Option<BlockPos> {
        self.find_blocks(nearest_to, block_states).next()
    }

    /// Find all the coordinates of a block in the world.
    ///
    /// This returns an iterator that yields the [`BlockPos`]s of blocks that
    /// are in the given block states.
    ///
    /// Note that this is sorted by `x+y+z` and not `x^2+y^2+z^2` for
    /// performance purposes. Blocks that are at an equal distance are not
    /// guaranteed to be returned in any specific order.
    pub fn find_blocks<'a>(
        &'a self,
        nearest_to: impl Into<BlockPos>,
        block_states: &'a BlockStates,
    ) -> FindBlocks<'a> {
        FindBlocks::new(nearest_to.into(), &self.chunks, block_states)
    }
}

pub struct FindBlocks<'a> {
    nearest_to: BlockPos,
    start_chunk: ChunkPos,
    chunk_iterator: ChunkIterator,
    chunks: &'a ChunkStorage,
    block_states: &'a BlockStates,

    queued: Vec<BlockPos>,
}

impl<'a> FindBlocks<'a> {
    pub fn new(
        nearest_to: BlockPos,
        chunks: &'a ChunkStorage,
        block_states: &'a BlockStates,
    ) -> Self {
        let start_chunk: ChunkPos = (&nearest_to).into();
        Self {
            nearest_to,
            start_chunk,
            chunk_iterator: ChunkIterator::new(start_chunk, 32),
            chunks,
            block_states,

            queued: Vec::new(),
        }
    }
}

impl Iterator for FindBlocks<'_> {
    type Item = BlockPos;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(queued) = self.queued.pop() {
            return Some(queued);
        }

        let mut found = Vec::new();

        let mut nearest_found_pos: Option<BlockPos> = None;
        let mut nearest_found_distance = 0;

        while let Some(chunk_pos) = self.chunk_iterator.next() {
            let Some(chunk) = self.chunks.get(&chunk_pos) else {
                // if the chunk isn't loaded then we skip it. we don't just
                // return since it *could* cause issues if there's a random
                // unloaded chunk and then more that are loaded. unlikely but
                // still something to consider, and it's not like this slows it
                // down much anyways.
                continue;
            };

            find_blocks_in_chunk(
                self.block_states,
                chunk_pos,
                &chunk.read(),
                self.chunks.min_y(),
                |this_block_pos| {
                    let this_block_distance = (self.nearest_to - this_block_pos).length_manhattan();

                    found.push((this_block_pos, this_block_distance));

                    if nearest_found_pos.is_none() || this_block_distance < nearest_found_distance {
                        nearest_found_pos = Some(this_block_pos);
                        nearest_found_distance = this_block_distance;
                    }
                },
            );

            if let Some(nearest_found_pos) = nearest_found_pos {
                // this is required because find_block searches chunk-by-chunk,
                // which can cause us to find blocks first that aren't actually
                // the closest
                let required_chunk_distance = u32::max(
                    u32::max(
                        (chunk_pos.x - self.start_chunk.x).unsigned_abs(),
                        (chunk_pos.z - self.start_chunk.z).unsigned_abs(),
                    ),
                    (self.nearest_to.y - nearest_found_pos.y)
                        .unsigned_abs()
                        .div_ceil(16),
                ) + 1;
                let nearest_chunk_distance = self.chunk_iterator.layer;

                // if we found a valid position and there's no chance that
                // there's something closer, return it
                if nearest_chunk_distance > required_chunk_distance {
                    break;
                }
            }
        }

        // sort so nearest is at the end
        found.sort_unstable_by_key(|(_, distance)| u32::MAX - distance);

        self.queued = found.into_iter().map(|(pos, _)| pos).collect();
        self.queued.pop()
    }
}

/// An optimized function for finding the block positions in a chunk that match
/// the given block states.
///
/// This is used internally by [`World::find_block`] and
/// [`World::find_blocks`].
pub fn find_blocks_in_chunk(
    block_states: &BlockStates,
    chunk_pos: ChunkPos,
    chunk: &Chunk,
    min_y: i32,
    mut cb: impl FnMut(BlockPos),
) {
    for (section_index, section) in chunk.sections.iter().enumerate() {
        let maybe_has_block = palette_maybe_has_block(&section.states.palette, block_states);
        if !maybe_has_block {
            continue;
        }

        for i in 0..4096 {
            let block_state = section.states.get_at_index(i);

            if block_states.contains(&block_state) {
                let section_pos = section.states.pos_from_index(i);
                let (x, y, z) = (
                    chunk_pos.x * 16 + (section_pos.x as i32),
                    min_y + (section_index * 16) as i32 + section_pos.y as i32,
                    chunk_pos.z * 16 + (section_pos.z as i32),
                );
                let this_block_pos = BlockPos { x, y, z };

                cb(this_block_pos);
            }
        }
    }
}

fn palette_maybe_has_block(palette: &Palette<BlockState>, block_states: &BlockStates) -> bool {
    match &palette {
        Palette::SingleValue(id) => block_states.contains(id),
        Palette::Linear(ids) => ids.iter().any(|id| block_states.contains(id)),
        Palette::Hashmap(ids) => ids.iter().any(|id| block_states.contains(id)),
        Palette::Global => true,
    }
}

#[cfg(test)]
mod tests {
    use azalea_registry::builtin::BlockKind;

    use super::*;
    use crate::{Chunk, chunk::partial::PartialChunkStorage};

    #[test]
    fn find_block() {
        let mut world = World::default();

        let chunk_storage = &mut world.chunks;
        let mut partial_chunk_storage = PartialChunkStorage::default();

        // block at (17, 0, 0) and (0, 18, 0)

        partial_chunk_storage.set(
            &ChunkPos { x: 0, z: 0 },
            Some(Chunk::default()),
            chunk_storage,
        );
        partial_chunk_storage.set(
            &ChunkPos { x: 1, z: 0 },
            Some(Chunk::default()),
            chunk_storage,
        );

        chunk_storage.set_block_state(BlockPos { x: 17, y: 0, z: 0 }, BlockKind::Stone.into());
        chunk_storage.set_block_state(BlockPos { x: 0, y: 18, z: 0 }, BlockKind::Stone.into());

        let pos = world.find_block(BlockPos { x: 0, y: 0, z: 0 }, &BlockKind::Stone.into());
        assert_eq!(pos, Some(BlockPos { x: 17, y: 0, z: 0 }));
    }

    #[test]
    fn find_blocks() {
        let mut world = World::default();

        let chunk_storage = &mut world.chunks;
        let mut partial_chunk_storage = PartialChunkStorage::default();

        // must be ordered by distance already
        let mut positions = Vec::new();
        for x in 0..50 {
            positions.push(BlockPos::new(x, 0, 0));
        }

        for &pos in &positions {
            let chunk_pos = ChunkPos::from(pos);
            if !partial_chunk_storage.has(&chunk_pos) {
                partial_chunk_storage.set(&chunk_pos, Some(Chunk::default()), chunk_storage);
            }
            chunk_storage.set_block_state(pos, BlockKind::Stone.into());
        }

        let result = world
            .find_blocks(BlockPos { x: 0, y: 0, z: 0 }, &BlockKind::Stone.into())
            .collect::<Box<[_]>>();
        let positions = positions.into();
        assert_eq!(result, positions);
    }

    #[test]
    fn find_block_next_to_chunk_border() {
        let mut world = World::default();

        let chunk_storage = &mut world.chunks;
        let mut partial_chunk_storage = PartialChunkStorage::default();

        // block at (-1, 0, 0) and (15, 0, 0)

        partial_chunk_storage.set(
            &ChunkPos { x: -1, z: 0 },
            Some(Chunk::default()),
            chunk_storage,
        );
        partial_chunk_storage.set(
            &ChunkPos { x: 0, z: 0 },
            Some(Chunk::default()),
            chunk_storage,
        );

        chunk_storage.set_block_state(BlockPos { x: -1, y: 0, z: 0 }, BlockKind::Stone.into());
        chunk_storage.set_block_state(BlockPos { x: 15, y: 0, z: 0 }, BlockKind::Stone.into());

        let pos = world.find_block(BlockPos { x: 0, y: 0, z: 0 }, &BlockKind::Stone.into());
        assert_eq!(pos, Some(BlockPos { x: -1, y: 0, z: 0 }));
    }
}
