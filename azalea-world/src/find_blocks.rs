use azalea_block::{BlockState, BlockStates};
use azalea_core::position::{BlockPos, ChunkPos};

use crate::{iterators::ChunkIterator, palette::Palette, ChunkStorage, Instance};

fn palette_maybe_has_block(palette: &Palette, block_states: &BlockStates) -> bool {
    match &palette {
        Palette::SingleValue(id) => block_states.contains(&BlockState { id: *id }),
        Palette::Linear(ids) => ids
            .iter()
            .any(|&id| block_states.contains(&BlockState { id })),
        Palette::Hashmap(ids) => ids
            .iter()
            .any(|&id| block_states.contains(&BlockState { id })),
        Palette::Global => true,
    }
}

impl Instance {
    /// Find the coordinates of a block in the world.
    ///
    /// Note that this is sorted by `x+y+z` and not `x^2+y^2+z^2` for
    /// optimization purposes.
    ///
    /// ```
    /// # fn example(client: &azalea_client::Client) {
    /// client.world().read().find_block(client.position(), &azalea_registry::Block::Chest.into());
    /// # }
    /// ```
    pub fn find_block(
        &self,
        nearest_to: impl Into<BlockPos>,
        block_states: &BlockStates,
    ) -> Option<BlockPos> {
        // iterate over every chunk in a 3d spiral pattern
        // and then check the palette for the block state

        let nearest_to: BlockPos = nearest_to.into();
        let start_chunk: ChunkPos = (&nearest_to).into();
        let mut iter = ChunkIterator::new(start_chunk, 32);

        let mut nearest_found_pos: Option<BlockPos> = None;
        let mut nearest_found_distance = 0;

        // we do `while` instead of `for` so we can access iter later
        while let Some(chunk_pos) = iter.next() {
            let Some(chunk) = self.chunks.get(&chunk_pos) else {
                // if the chunk isn't loaded then we skip it.
                // we don't just return since it *could* cause issues if there's a random
                // unloaded chunk and then more that are loaded.
                // unlikely but still something to consider, and it's not like this slows it
                // down much anyways.
                continue;
            };

            for (section_index, section) in chunk.read().sections.iter().enumerate() {
                let maybe_has_block =
                    palette_maybe_has_block(&section.states.palette, block_states);
                if !maybe_has_block {
                    continue;
                }

                for i in 0..4096 {
                    let block_state = section.states.get_at_index(i);
                    let block_state = BlockState { id: block_state };

                    if block_states.contains(&block_state) {
                        let (section_x, section_y, section_z) = section.states.coords_from_index(i);
                        let (x, y, z) = (
                            chunk_pos.x * 16 + (section_x as i32),
                            self.chunks.min_y + (section_index * 16) as i32 + section_y as i32,
                            chunk_pos.z * 16 + (section_z as i32),
                        );
                        let this_block_pos = BlockPos { x, y, z };
                        let this_block_distance = (nearest_to - this_block_pos).length_manhattan();
                        // only update if it's closer
                        if nearest_found_pos.is_none()
                            || this_block_distance < nearest_found_distance
                        {
                            nearest_found_pos = Some(this_block_pos);
                            nearest_found_distance = this_block_distance;
                        }
                    }
                }
            }

            if let Some(nearest_found_pos) = nearest_found_pos {
                // this is required because find_block searches chunk-by-chunk, which can cause
                // us to find blocks first that aren't actually the closest
                let required_chunk_distance = u32::max(
                    u32::max(
                        (chunk_pos.x - start_chunk.x).unsigned_abs(),
                        (chunk_pos.z - start_chunk.z).unsigned_abs(),
                    ),
                    (nearest_to.y - nearest_found_pos.y)
                        .unsigned_abs()
                        .div_ceil(16),
                ) + 1;
                let nearest_chunk_distance = iter.layer;

                // if we found the position and there's no chance there's something closer,
                // return it
                if nearest_chunk_distance > required_chunk_distance {
                    return Some(nearest_found_pos);
                }
            }
        }

        if nearest_found_pos.is_some() {
            nearest_found_pos
        } else {
            None
        }
    }

    /// Find all the coordinates of a block in the world.
    ///
    /// This returns an iterator that yields the [`BlockPos`]s of blocks that
    /// are in the given block states. It's sorted by `x+y+z`.
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

impl<'a> Iterator for FindBlocks<'a> {
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
                // if the chunk isn't loaded then we skip it.
                // we don't just return since it *could* cause issues if there's a random
                // unloaded chunk and then more that are loaded.
                // unlikely but still something to consider, and it's not like this slows it
                // down much anyways.
                continue;
            };

            for (section_index, section) in chunk.read().sections.iter().enumerate() {
                let maybe_has_block =
                    palette_maybe_has_block(&section.states.palette, self.block_states);
                if !maybe_has_block {
                    continue;
                }

                for i in 0..4096 {
                    let block_state = section.states.get_at_index(i);
                    let block_state = BlockState { id: block_state };

                    if self.block_states.contains(&block_state) {
                        let (section_x, section_y, section_z) = section.states.coords_from_index(i);
                        let (x, y, z) = (
                            chunk_pos.x * 16 + (section_x as i32),
                            self.chunks.min_y + (section_index * 16) as i32 + section_y as i32,
                            chunk_pos.z * 16 + (section_z as i32),
                        );
                        let this_block_pos = BlockPos { x, y, z };
                        let this_block_distance =
                            (self.nearest_to - this_block_pos).length_manhattan();

                        found.push((this_block_pos, this_block_distance));

                        if nearest_found_pos.is_none()
                            || this_block_distance < nearest_found_distance
                        {
                            nearest_found_pos = Some(this_block_pos);
                            nearest_found_distance = this_block_distance;
                        }
                    }
                }
            }

            if let Some(nearest_found_pos) = nearest_found_pos {
                // this is required because find_block searches chunk-by-chunk, which can cause
                // us to find blocks first that aren't actually the closest
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

                // if we found the position and there's no chance there's something closer,
                // return it
                if nearest_chunk_distance > required_chunk_distance {
                    // sort so nearest is at the end
                    found.sort_unstable_by_key(|(_, distance)| u32::MAX - distance);

                    self.queued = found.into_iter().map(|(pos, _)| pos).collect();
                    return self.queued.pop();
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use azalea_registry::Block;

    use super::*;
    use crate::{Chunk, PartialChunkStorage};

    #[test]
    fn find_block() {
        let mut instance = Instance::default();

        let chunk_storage = &mut instance.chunks;
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

        chunk_storage.set_block_state(&BlockPos { x: 17, y: 0, z: 0 }, Block::Stone.into());
        chunk_storage.set_block_state(&BlockPos { x: 0, y: 18, z: 0 }, Block::Stone.into());

        let pos = instance.find_block(BlockPos { x: 0, y: 0, z: 0 }, &Block::Stone.into());
        assert_eq!(pos, Some(BlockPos { x: 17, y: 0, z: 0 }));
    }

    #[test]
    fn find_block_next_to_chunk_border() {
        let mut instance = Instance::default();

        let chunk_storage = &mut instance.chunks;
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

        chunk_storage.set_block_state(&BlockPos { x: -1, y: 0, z: 0 }, Block::Stone.into());
        chunk_storage.set_block_state(&BlockPos { x: 15, y: 0, z: 0 }, Block::Stone.into());

        let pos = instance.find_block(BlockPos { x: 0, y: 0, z: 0 }, &Block::Stone.into());
        assert_eq!(pos, Some(BlockPos { x: -1, y: 0, z: 0 }));
    }
}
