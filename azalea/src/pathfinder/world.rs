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
use rustc_hash::FxHashMap;

use super::mining::MiningCache;

/// An efficient representation of the world used for the pathfinder.
pub struct CachedWorld {
    min_y: i32,
    world_lock: Arc<RwLock<Instance>>,

    // we store `PalettedContainer`s instead of `Chunk`s or `Section`s because it doesn't contain
    // any unnecessary data like heightmaps or biomes.
    cached_chunks: RefCell<Vec<(ChunkPos, Vec<azalea_world::palette::PalettedContainer>)>>,
    last_chunk_cache_index: RefCell<Option<usize>>,

    cached_blocks: UnsafeCell<CachedSections>,

    cached_mining_costs: RefCell<FxHashMap<BlockPos, f32>>,
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
            last_chunk_cache_index: Default::default(),
            cached_blocks: Default::default(),
            cached_mining_costs: Default::default(),
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
        if section_pos.y * 16 < self.min_y {
            // y position is out of bounds
            return None;
        }

        let chunk_pos = ChunkPos::from(section_pos);
        let section_index =
            azalea_world::chunk_storage::section_index(section_pos.y * 16, self.min_y) as usize;

        let mut cached_chunks = self.cached_chunks.borrow_mut();

        // optimization: avoid doing the iter lookup if the last chunk we looked up is
        // the same
        if let Some(last_chunk_cache_index) = *self.last_chunk_cache_index.borrow() {
            if cached_chunks[last_chunk_cache_index].0 == chunk_pos {
                // don't bother with the iter lookup
                let sections = &cached_chunks[last_chunk_cache_index].1;
                if section_index >= sections.len() {
                    // y position is out of bounds
                    return None;
                };
                let section: &azalea_world::palette::PalettedContainer = &sections[section_index];
                return Some(f(section));
            }
        }

        // get section from cache
        if let Some((chunk_index, sections)) =
            cached_chunks
                .iter()
                .enumerate()
                .find_map(|(i, (pos, sections))| {
                    if *pos == chunk_pos {
                        Some((i, sections))
                    } else {
                        None
                    }
                })
        {
            if section_index >= sections.len() {
                // y position is out of bounds
                return None;
            };
            *self.last_chunk_cache_index.borrow_mut() = Some(chunk_index);
            let section: &azalea_world::palette::PalettedContainer = &sections[section_index];
            return Some(f(section));
        }

        let world = self.world_lock.read();
        let chunk = world.chunks.get(&chunk_pos)?;
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

    /// Returns how much it costs to break this block. Returns 0 if the block is
    /// already passable.
    pub fn cost_for_breaking_block(&self, pos: BlockPos, mining_cache: &MiningCache) -> f32 {
        let mut cached_mining_costs = self.cached_mining_costs.borrow_mut();

        if let Some(&cost) = cached_mining_costs.get(&pos) {
            return cost;
        }

        let cost = self.uncached_cost_for_breaking_block(pos, mining_cache);
        cached_mining_costs.insert(pos, cost);
        cost
    }

    fn uncached_cost_for_breaking_block(&self, pos: BlockPos, mining_cache: &MiningCache) -> f32 {
        if self.is_block_passable(pos) {
            // if the block is passable then it doesn't need to be broken
            return 0.;
        }

        let (section_pos, section_block_pos) =
            (ChunkSectionPos::from(pos), ChunkSectionBlockPos::from(pos));

        // we use this as an optimization to avoid getting the section again if the
        // block is in the same section
        let up_is_in_same_section = section_block_pos.y != 15;
        let north_is_in_same_section = section_block_pos.z != 0;
        let east_is_in_same_section = section_block_pos.x != 15;
        let south_is_in_same_section = section_block_pos.z != 15;
        let west_is_in_same_section = section_block_pos.x != 0;

        let Some(mining_cost) = self.with_section(section_pos, |section| {
            let block_state =
                BlockState::try_from(section.get_at_index(u16::from(section_block_pos) as usize))
                    .unwrap_or_default();
            let mining_cost = mining_cache.cost_for(block_state);

            if mining_cost == f32::INFINITY {
                // the block is unbreakable
                return f32::INFINITY;
            }

            // if there's a falling block or liquid above this block, abort
            if up_is_in_same_section {
                let up_block = BlockState::try_from(
                    section.get_at_index(u16::from(section_block_pos.up(1)) as usize),
                )
                .unwrap_or_default();
                if mining_cache.is_liquid(up_block) || mining_cache.is_falling_block(up_block) {
                    return f32::INFINITY;
                }
            }

            // if there's a liquid to the north of this block, abort
            if north_is_in_same_section {
                let north_block = BlockState::try_from(
                    section.get_at_index(u16::from(section_block_pos.north(1)) as usize),
                )
                .unwrap_or_default();
                if mining_cache.is_liquid(north_block) {
                    return f32::INFINITY;
                }
            }

            // liquid to the east
            if east_is_in_same_section {
                let east_block = BlockState::try_from(
                    section.get_at_index(u16::from(section_block_pos.east(1)) as usize),
                )
                .unwrap_or_default();
                if mining_cache.is_liquid(east_block) {
                    return f32::INFINITY;
                }
            }

            // liquid to the south
            if south_is_in_same_section {
                let south_block = BlockState::try_from(
                    section.get_at_index(u16::from(section_block_pos.south(1)) as usize),
                )
                .unwrap_or_default();
                if mining_cache.is_liquid(south_block) {
                    return f32::INFINITY;
                }
            }

            // liquid to the west
            if west_is_in_same_section {
                let west_block = BlockState::try_from(
                    section.get_at_index(u16::from(section_block_pos.west(1)) as usize),
                )
                .unwrap_or_default();
                if mining_cache.is_liquid(west_block) {
                    return f32::INFINITY;
                }
            }

            // the block is probably safe to break, we'll have to check the adjacent blocks
            // that weren't in the same section next though
            mining_cost
        }) else {
            // the chunk isn't loaded
            let cost = if self.is_block_solid(pos) {
                // assume it's unbreakable if it's solid and out of render distance
                f32::INFINITY
            } else {
                0.
            };
            return cost;
        };

        if mining_cost == f32::INFINITY {
            // the block is unbreakable
            return f32::INFINITY;
        }

        let check_should_avoid_this_block = |pos: BlockPos, check: &dyn Fn(BlockState) -> bool| {
            let block_state = self
                .with_section(ChunkSectionPos::from(pos), |section| {
                    BlockState::try_from(
                        section.get_at_index(u16::from(ChunkSectionBlockPos::from(pos)) as usize),
                    )
                    .unwrap_or_default()
                })
                .unwrap_or_default();
            check(block_state)
        };

        // check the adjacent blocks that weren't in the same section
        if !up_is_in_same_section
            && check_should_avoid_this_block(pos.up(1), &|b| {
                mining_cache.is_liquid(b) || mining_cache.is_falling_block(b)
            })
        {
            return f32::INFINITY;
        }
        if !north_is_in_same_section
            && check_should_avoid_this_block(pos.north(1), &|b| mining_cache.is_liquid(b))
        {
            return f32::INFINITY;
        }
        if !east_is_in_same_section
            && check_should_avoid_this_block(pos.east(1), &|b| mining_cache.is_liquid(b))
        {
            return f32::INFINITY;
        }
        if !south_is_in_same_section
            && check_should_avoid_this_block(pos.south(1), &|b| mining_cache.is_liquid(b))
        {
            return f32::INFINITY;
        }
        if !west_is_in_same_section
            && check_should_avoid_this_block(pos.west(1), &|b| mining_cache.is_liquid(b))
        {
            return f32::INFINITY;
        }

        mining_cost
    }

    /// Whether this block and the block above are passable
    pub fn is_passable(&self, pos: BlockPos) -> bool {
        self.is_block_passable(pos) && self.is_block_passable(pos.up(1))
    }

    pub fn cost_for_passing(&self, pos: BlockPos, mining_cache: &MiningCache) -> f32 {
        self.cost_for_breaking_block(pos, mining_cache)
            + self.cost_for_breaking_block(pos.up(1), mining_cache)
    }

    /// Whether we can stand in this position. Checks if the block below is
    /// solid, and that the two blocks above that are passable.
    pub fn is_standable(&self, pos: BlockPos) -> bool {
        self.is_block_solid(pos.down(1)) && self.is_passable(pos)
    }

    pub fn cost_for_standing(&self, pos: BlockPos, mining_cache: &MiningCache) -> f32 {
        if !self.is_block_solid(pos.down(1)) {
            return f32::INFINITY;
        }
        self.cost_for_passing(pos, mining_cache)
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
pub fn is_block_state_passable(block: BlockState) -> bool {
    // i already tried optimizing this by having it cache in an IntMap/FxHashMap but
    // it wasn't measurably faster

    if block.is_air() {
        // fast path
        return true;
    }
    if !block.is_shape_empty() {
        return false;
    }
    let registry_block = azalea_registry::Block::from(block);
    if registry_block == azalea_registry::Block::Water {
        return false;
    }
    if block
        .property::<azalea_block::properties::Waterlogged>()
        .unwrap_or_default()
    {
        return false;
    }
    if registry_block == azalea_registry::Block::Lava {
        return false;
    }
    // block.waterlogged currently doesn't account for seagrass and some other water
    // blocks
    if block == azalea_registry::Block::Seagrass.into() {
        return false;
    }

    // don't walk into fire
    if registry_block == azalea_registry::Block::Fire
        || registry_block == azalea_registry::Block::SoulFire
    {
        return false;
    }

    true
}

/// whether this block has a solid hitbox (i.e. we can stand on it)
pub fn is_block_state_solid(block: BlockState) -> bool {
    if block.is_air() {
        // fast path
        return false;
    }
    block.is_shape_full()
}

#[cfg(test)]
mod tests {

    use azalea_world::{Chunk, ChunkStorage, PartialInstance};

    use super::*;

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
