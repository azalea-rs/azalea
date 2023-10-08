use std::{
    cell::{RefCell, UnsafeCell},
    sync::Arc,
};

use azalea_block::BlockState;
use azalea_core::{
    bitset::FixedBitSet,
    position::{BlockPos, ChunkPos, ChunkSectionBlockPos, ChunkSectionPos},
};
use azalea_physics::collision::BlockWithShape;
use azalea_world::Instance;
use parking_lot::RwLock;

/// An efficient representation of the world used for the pathfinder.
pub struct CachedWorld {
    min_y: i32,
    world_lock: Arc<RwLock<Instance>>,

    // we store `PalettedContainer`s instead of `Chunk`s or `Section`s because it doesn't contain
    // any unnecessary data like heightmaps or biomes.
    cached_chunks: RefCell<Vec<(ChunkPos, Vec<azalea_world::palette::PalettedContainer>)>>,

    cached_blocks: UnsafeCell<CachedSections>,
}

#[derive(Default)]
pub struct CachedSections {
    pub last_index: usize,
    pub second_last_index: usize,
    pub sections: Vec<CachedSection>,
}

impl CachedSections {
    #[inline]
    pub fn get_mut(&mut self, pos: ChunkSectionPos) -> Option<&mut CachedSection> {
        if let Some(last_item) = self.sections.get(self.last_index) {
            if last_item.pos == pos {
                return Some(&mut self.sections[self.last_index]);
            } else if let Some(second_last_item) = self.sections.get(self.second_last_index) {
                if second_last_item.pos == pos {
                    return Some(&mut self.sections[self.second_last_index]);
                }
            }
        }

        let index = self
            .sections
            .binary_search_by(|section| section.pos.cmp(&pos))
            .ok();

        if let Some(index) = index {
            self.second_last_index = self.last_index;
            self.last_index = index;
            return Some(&mut self.sections[index]);
        }
        None
    }

    #[inline]
    pub fn insert(&mut self, section: CachedSection) {
        // self.sections.push(section);
        // self.sections.sort_unstable_by(|a, b| a.pos.cmp(&b.pos));
        let index = self
            .sections
            .binary_search_by(|s| s.pos.cmp(&section.pos))
            .unwrap_or_else(|e| e);
        self.sections.insert(index, section);
    }
}

pub struct CachedSection {
    pub pos: ChunkSectionPos,
    pub passable_bitset: FixedBitSet<4096>,
    pub solid_bitset: FixedBitSet<4096>,
}

impl CachedWorld {
    pub fn new(world_lock: Arc<RwLock<Instance>>) -> Self {
        let min_y = world_lock.read().chunks.min_y;
        Self {
            min_y,
            world_lock,
            cached_chunks: Default::default(),
            cached_blocks: Default::default(),
        }
    }

    // ```
    // fn get_block_state(&self, pos: BlockPos) -> Option<BlockState> {
    //     self.with_section(ChunkSectionPos::from(pos), |section| {
    //         let state = section.get(pos.x as usize, pos.y as usize, pos.z as usize);
    //         BlockState::try_from(state).unwrap_or(BlockState::AIR)
    //     })
    // }
    // ```

    fn with_section<T>(
        &self,
        section_pos: ChunkSectionPos,
        f: impl FnOnce(&azalea_world::palette::PalettedContainer) -> T,
    ) -> Option<T> {
        let chunk_pos = ChunkPos::from(section_pos);
        let section_index =
            azalea_world::chunk_storage::section_index(section_pos.y * 16, self.min_y) as usize;

        let mut cached_chunks = self.cached_chunks.borrow_mut();

        // get section from cache
        if let Some(sections) = cached_chunks.iter().find_map(|(pos, sections)| {
            if *pos == chunk_pos {
                Some(sections)
            } else {
                None
            }
        }) {
            if section_index >= sections.len() {
                // y position is out of bounds
                return None;
            };
            let section: &azalea_world::palette::PalettedContainer = &sections[section_index];
            return Some(f(section));
        }

        let world = self.world_lock.read();
        let Some(chunk) = world.chunks.get(&chunk_pos) else {
            return None;
        };
        let chunk = chunk.read();

        let sections: Vec<azalea_world::palette::PalettedContainer> = chunk
            .sections
            .iter()
            .map(|section| section.states.clone())
            .collect();

        if section_index >= sections.len() {
            // y position is out of bounds
            return None;
        };

        let section = &sections[section_index];
        let r = f(section);

        // add the sections to the chunk cache
        cached_chunks.push((chunk_pos, sections));

        Some(r)
    }

    fn calculate_bitsets_for_section(&self, section_pos: ChunkSectionPos) -> Option<CachedSection> {
        self.with_section(section_pos, |section| {
            let mut passable_bitset = FixedBitSet::<4096>::new();
            let mut solid_bitset = FixedBitSet::<4096>::new();
            for i in 0..4096 {
                let block_state_id = section.get_at_index(i);
                let block_state = BlockState::try_from(block_state_id).unwrap_or(BlockState::AIR);
                if is_block_state_passable(block_state) {
                    passable_bitset.set(i);
                }
                if is_block_state_solid(block_state) {
                    solid_bitset.set(i);
                }
            }
            CachedSection {
                pos: section_pos,
                passable_bitset,
                solid_bitset,
            }
        })
    }

    pub fn is_block_passable(&self, pos: BlockPos) -> bool {
        let (section_pos, section_block_pos) =
            (ChunkSectionPos::from(pos), ChunkSectionBlockPos::from(pos));
        let index = u16::from(section_block_pos) as usize;
        // SAFETY: we're only accessing this from one thread
        let cached_blocks = unsafe { &mut *self.cached_blocks.get() };
        if let Some(cached) = cached_blocks.get_mut(section_pos) {
            return cached.passable_bitset.index(index);
        }

        let Some(cached) = self.calculate_bitsets_for_section(section_pos) else {
            return false;
        };
        let passable = cached.passable_bitset.index(index);
        cached_blocks.insert(cached);
        passable
    }

    pub fn is_block_solid(&self, pos: BlockPos) -> bool {
        let (section_pos, section_block_pos) =
            (ChunkSectionPos::from(pos), ChunkSectionBlockPos::from(pos));
        let index = u16::from(section_block_pos) as usize;
        // SAFETY: we're only accessing this from one thread
        let cached_blocks = unsafe { &mut *self.cached_blocks.get() };
        if let Some(cached) = cached_blocks.get_mut(section_pos) {
            return cached.solid_bitset.index(index);
        }

        let Some(cached) = self.calculate_bitsets_for_section(section_pos) else {
            return false;
        };
        let solid = cached.solid_bitset.index(index);
        cached_blocks.insert(cached);
        solid
    }

    /// Whether this block and the block above are passable
    pub fn is_passable(&self, pos: BlockPos) -> bool {
        self.is_block_passable(pos) && self.is_block_passable(pos.up(1))
    }

    /// Whether we can stand in this position. Checks if the block below is
    /// solid, and that the two blocks above that are passable.

    pub fn is_standable(&self, pos: BlockPos) -> bool {
        self.is_block_solid(pos.down(1)) && self.is_passable(pos)
    }

    /// Get the amount of air blocks until the next solid block below this one.
    pub fn fall_distance(&self, pos: BlockPos) -> u32 {
        let mut distance = 0;
        let mut current_pos = pos.down(1);
        while self.is_block_passable(current_pos) {
            distance += 1;
            current_pos = current_pos.down(1);

            if current_pos.y < self.min_y {
                return u32::MAX;
            }
        }
        distance
    }
}

/// whether this block is passable
fn is_block_state_passable(block: BlockState) -> bool {
    if block.is_air() {
        // fast path
        return true;
    }
    if !block.is_shape_empty() {
        return false;
    }
    if block == azalea_registry::Block::Water.into() {
        return false;
    }
    if block.waterlogged() {
        return false;
    }
    if block == azalea_registry::Block::Lava.into() {
        return false;
    }
    // block.waterlogged currently doesn't account for seagrass and some other water
    // blocks
    if block == azalea_registry::Block::Seagrass.into() {
        return false;
    }

    true
}

/// whether this block has a solid hitbox (i.e. we can stand on it)
fn is_block_state_solid(block: BlockState) -> bool {
    if block.is_air() {
        // fast path
        return false;
    }
    block.is_shape_full()
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use azalea_block::BlockState;
    use azalea_core::position::ChunkPos;
    use azalea_world::{Chunk, ChunkStorage, PartialInstance};
    use parking_lot::RwLock;

    #[test]
    fn test_is_passable() {
        let mut partial_world = PartialInstance::default();
        let mut world = ChunkStorage::default();

        partial_world
            .chunks
            .set(&ChunkPos { x: 0, z: 0 }, Some(Chunk::default()), &mut world);
        partial_world.chunks.set_block_state(
            &BlockPos::new(0, 0, 0),
            azalea_registry::Block::Stone.into(),
            &world,
        );
        partial_world
            .chunks
            .set_block_state(&BlockPos::new(0, 1, 0), BlockState::AIR, &world);

        let ctx = CachedWorld::new(Arc::new(RwLock::new(world.into())));
        assert!(!ctx.is_block_passable(BlockPos::new(0, 0, 0)));
        assert!(ctx.is_block_passable(BlockPos::new(0, 1, 0),));
    }

    #[test]
    fn test_is_solid() {
        let mut partial_world = PartialInstance::default();
        let mut world = ChunkStorage::default();
        partial_world
            .chunks
            .set(&ChunkPos { x: 0, z: 0 }, Some(Chunk::default()), &mut world);
        partial_world.chunks.set_block_state(
            &BlockPos::new(0, 0, 0),
            azalea_registry::Block::Stone.into(),
            &world,
        );
        partial_world
            .chunks
            .set_block_state(&BlockPos::new(0, 1, 0), BlockState::AIR, &world);

        let ctx = CachedWorld::new(Arc::new(RwLock::new(world.into())));
        assert!(ctx.is_block_solid(BlockPos::new(0, 0, 0)));
        assert!(!ctx.is_block_solid(BlockPos::new(0, 1, 0)));
    }

    #[test]
    fn test_is_standable() {
        let mut partial_world = PartialInstance::default();
        let mut world = ChunkStorage::default();
        partial_world
            .chunks
            .set(&ChunkPos { x: 0, z: 0 }, Some(Chunk::default()), &mut world);
        partial_world.chunks.set_block_state(
            &BlockPos::new(0, 0, 0),
            azalea_registry::Block::Stone.into(),
            &world,
        );
        partial_world
            .chunks
            .set_block_state(&BlockPos::new(0, 1, 0), BlockState::AIR, &world);
        partial_world
            .chunks
            .set_block_state(&BlockPos::new(0, 2, 0), BlockState::AIR, &world);
        partial_world
            .chunks
            .set_block_state(&BlockPos::new(0, 3, 0), BlockState::AIR, &world);

        let ctx = CachedWorld::new(Arc::new(RwLock::new(world.into())));
        assert!(ctx.is_standable(BlockPos::new(0, 1, 0)));
        assert!(!ctx.is_standable(BlockPos::new(0, 0, 0)));
        assert!(!ctx.is_standable(BlockPos::new(0, 2, 0)));
    }
}
