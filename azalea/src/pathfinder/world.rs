use std::{
    cell::{RefCell, UnsafeCell},
    sync::Arc,
};

use azalea_block::{BlockState, properties};
use azalea_core::{
    bitset::FastFixedBitSet,
    position::{BlockPos, ChunkPos, ChunkSectionBlockPos, ChunkSectionPos},
};
use azalea_physics::collision::BlockWithShape;
use azalea_registry::{Block, tags};
use azalea_world::{Instance, palette::PalettedContainer};
use parking_lot::RwLock;

use super::{mining::MiningCache, rel_block_pos::RelBlockPos};

/// An efficient representation of the world used for the pathfinder.
pub struct CachedWorld {
    /// The origin that the [`RelBlockPos`] types will be relative to.
    ///
    /// This is for an optimization that reduces the size of the block positions
    /// that are used by the pathfinder.
    origin: BlockPos,

    min_y: i32,
    world_lock: Arc<RwLock<Instance>>,

    // we store `PalettedContainer`s instead of `Chunk`s or `Section`s because it doesn't contain
    // any unnecessary data like heightmaps or biomes.
    cached_chunks: RefCell<
        Vec<(
            ChunkPos,
            Vec<azalea_world::palette::PalettedContainer<BlockState>>,
        )>,
    >,
    last_chunk_cache_index: RefCell<Option<usize>>,

    cached_blocks: UnsafeCell<CachedSections>,

    cached_mining_costs: UnsafeCell<Box<[(RelBlockPos, f32)]>>,
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
            } else if let Some(second_last_item) = self.sections.get(self.second_last_index)
                && second_last_item.pos == pos
            {
                return Some(&mut self.sections[self.second_last_index]);
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
    /// Blocks that we can fully pass through (like air).
    pub passable_bitset: FastFixedBitSet<4096>,
    /// Blocks that we can stand on and do parkour from.
    pub solid_bitset: FastFixedBitSet<4096>,
    /// Blocks that we can stand on but might not be able to parkour from.
    pub standable_bitset: FastFixedBitSet<4096>,
}

impl CachedWorld {
    pub fn new(world_lock: Arc<RwLock<Instance>>, origin: BlockPos) -> Self {
        let min_y = world_lock.read().chunks.min_y;
        Self {
            origin,
            min_y,
            world_lock,
            cached_chunks: Default::default(),
            last_chunk_cache_index: Default::default(),
            cached_blocks: Default::default(),
            // this uses about 12mb of memory. it *really* helps though.
            cached_mining_costs: UnsafeCell::new(
                vec![(RelBlockPos::new(i16::MAX, i32::MAX, i16::MAX), 0.); 2usize.pow(20)]
                    .into_boxed_slice(),
            ),
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
        f: impl FnOnce(&azalea_world::palette::PalettedContainer<BlockState>) -> T,
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
        if let Some(last_chunk_cache_index) = *self.last_chunk_cache_index.borrow()
            && cached_chunks[last_chunk_cache_index].0 == chunk_pos
        {
            // don't bother with the iter lookup
            let sections = &cached_chunks[last_chunk_cache_index].1;
            if section_index >= sections.len() {
                // y position is out of bounds
                return None;
            };
            let section = &sections[section_index];
            return Some(f(section));
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
            let section = &sections[section_index];
            return Some(f(section));
        }

        let world = self.world_lock.read();
        let chunk = world.chunks.get(&chunk_pos)?;
        let chunk = chunk.read();

        let sections = chunk
            .sections
            .iter()
            .map(|section| section.states.clone())
            .collect::<Vec<PalettedContainer<BlockState>>>();

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
            let mut passable_bitset = FastFixedBitSet::<4096>::new();
            let mut solid_bitset = FastFixedBitSet::<4096>::new();
            let mut standable_bitset = FastFixedBitSet::<4096>::new();
            for i in 0..4096 {
                let block_state = section.get_at_index(i);
                if is_block_state_passable(block_state) {
                    passable_bitset.set(i);
                }
                if is_block_state_solid(block_state) {
                    solid_bitset.set(i);
                }
                if is_block_state_standable(block_state) {
                    standable_bitset.set(i);
                }
            }
            CachedSection {
                pos: section_pos,
                passable_bitset,
                solid_bitset,
                standable_bitset,
            }
        })
    }

    pub fn is_block_passable(&self, pos: RelBlockPos) -> bool {
        self.is_block_pos_passable(pos.apply(self.origin))
    }

    fn is_block_pos_passable(&self, pos: BlockPos) -> bool {
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

    /// Get the block state at the given position.
    ///
    /// This is relatively slow, so you should avoid it whenever possible.
    pub fn get_block_state(&self, pos: RelBlockPos) -> BlockState {
        self.get_block_state_at_pos(pos.apply(self.origin))
    }

    fn get_block_state_at_pos(&self, pos: BlockPos) -> BlockState {
        let (section_pos, section_block_pos) =
            (ChunkSectionPos::from(pos), ChunkSectionBlockPos::from(pos));
        let index = u16::from(section_block_pos) as usize;

        self.with_section(section_pos, |section| section.get_at_index(index))
            .unwrap_or_default()
    }

    pub fn is_block_solid(&self, pos: RelBlockPos) -> bool {
        self.is_block_pos_solid(pos.apply(self.origin))
    }
    pub fn is_block_standable(&self, pos: RelBlockPos) -> bool {
        self.is_block_pos_standable(pos.apply(self.origin))
    }

    fn is_block_pos_solid(&self, pos: BlockPos) -> bool {
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
    fn is_block_pos_standable(&self, pos: BlockPos) -> bool {
        let (section_pos, section_block_pos) =
            (ChunkSectionPos::from(pos), ChunkSectionBlockPos::from(pos));
        let index = u16::from(section_block_pos) as usize;
        // SAFETY: we're only accessing this from one thread
        let cached_blocks = unsafe { &mut *self.cached_blocks.get() };
        if let Some(cached) = cached_blocks.get_mut(section_pos) {
            return cached.standable_bitset.index(index);
        }

        let Some(cached) = self.calculate_bitsets_for_section(section_pos) else {
            return false;
        };
        let solid = cached.standable_bitset.index(index);
        cached_blocks.insert(cached);
        solid
    }

    /// Returns how much it costs to break this block.
    ///
    /// Returns 0 if the block is already passable.
    pub fn cost_for_breaking_block(&self, pos: RelBlockPos, mining_cache: &MiningCache) -> f32 {
        // SAFETY: pathfinding is single-threaded
        let cached_mining_costs = unsafe { &mut *self.cached_mining_costs.get() };
        // 20 bits total:
        // 8 bits for x, 4 bits for y, 8 bits for z
        let hash_index = ((pos.x as usize & 0xff) << 12)
            | ((pos.y as usize & 0xf) << 8)
            | (pos.z as usize & 0xff);
        debug_assert!(hash_index < 1048576);
        let &(cached_pos, potential_cost) =
            unsafe { cached_mining_costs.get_unchecked(hash_index) };
        if cached_pos == pos {
            return potential_cost;
        }

        let cost = self.uncached_cost_for_breaking_block(pos, mining_cache);
        unsafe {
            *cached_mining_costs.get_unchecked_mut(hash_index) = (pos, cost);
        };

        cost
    }

    fn uncached_cost_for_breaking_block(
        &self,
        pos: RelBlockPos,
        mining_cache: &MiningCache,
    ) -> f32 {
        if self.is_block_passable(pos) {
            // if the block is passable then it doesn't need to be broken
            return 0.;
        }

        let pos = pos.apply(self.origin);

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
            let block_state = section.get_at_index(u16::from(section_block_pos) as usize);
            let mining_cost = mining_cache.cost_for(block_state);

            if mining_cost == f32::INFINITY {
                // the block is unbreakable
                return f32::INFINITY;
            }

            // if there's a falling block or liquid above this block, abort
            if up_is_in_same_section {
                let up_block = section.get_at_index(u16::from(section_block_pos.up(1)) as usize);
                if mining_cache.is_liquid(up_block) || mining_cache.is_falling_block(up_block) {
                    return f32::INFINITY;
                }
            }

            // if there's a liquid to the north of this block, abort
            if north_is_in_same_section {
                let north_block =
                    section.get_at_index(u16::from(section_block_pos.north(1)) as usize);
                if mining_cache.is_liquid(north_block) {
                    return f32::INFINITY;
                }
            }

            // liquid to the east
            if east_is_in_same_section {
                let east_block =
                    section.get_at_index(u16::from(section_block_pos.east(1)) as usize);
                if mining_cache.is_liquid(east_block) {
                    return f32::INFINITY;
                }
            }

            // liquid to the south
            if south_is_in_same_section {
                let south_block =
                    section.get_at_index(u16::from(section_block_pos.south(1)) as usize);
                if mining_cache.is_liquid(south_block) {
                    return f32::INFINITY;
                }
            }

            // liquid to the west
            if west_is_in_same_section {
                let west_block =
                    section.get_at_index(u16::from(section_block_pos.west(1)) as usize);
                if mining_cache.is_liquid(west_block) {
                    return f32::INFINITY;
                }
            }

            // the block is probably safe to break, we'll have to check the adjacent blocks
            // that weren't in the same section next though
            mining_cost
        }) else {
            // the chunk isn't loaded
            let cost = if self.is_block_pos_solid(pos) {
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
                    section.get_at_index(u16::from(ChunkSectionBlockPos::from(pos)) as usize)
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
    pub fn is_passable(&self, pos: RelBlockPos) -> bool {
        self.is_passable_at_block_pos(pos.apply(self.origin))
    }
    fn is_passable_at_block_pos(&self, pos: BlockPos) -> bool {
        self.is_block_pos_passable(pos) && self.is_block_pos_passable(pos.up(1))
    }

    pub fn cost_for_passing(&self, pos: RelBlockPos, mining_cache: &MiningCache) -> f32 {
        self.cost_for_breaking_block(pos, mining_cache)
            + self.cost_for_breaking_block(pos.up(1), mining_cache)
    }

    /// Whether we can stand in this position.
    ///
    /// Checks if the block below is solid, and that the two blocks above that
    /// are passable.
    pub fn is_standable(&self, pos: RelBlockPos) -> bool {
        self.is_standable_at_block_pos(pos.apply(self.origin))
    }
    fn is_standable_at_block_pos(&self, pos: BlockPos) -> bool {
        self.is_block_pos_standable(pos.down(1)) && self.is_passable_at_block_pos(pos)
    }

    pub fn cost_for_standing(&self, pos: RelBlockPos, mining_cache: &MiningCache) -> f32 {
        if !self.is_block_standable(pos.down(1)) {
            return f32::INFINITY;
        }
        self.cost_for_passing(pos, mining_cache)
    }

    /// Get the amount of air blocks until the next solid block below this one.
    pub fn fall_distance(&self, pos: RelBlockPos) -> u32 {
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

    pub fn origin(&self) -> BlockPos {
        self.origin
    }
}

/// Whether our client could pass through this block.
pub fn is_block_state_passable(block_state: BlockState) -> bool {
    // i already tried optimizing this by having it cache in an IntMap/FxHashMap but
    // it wasn't measurably faster

    if block_state.is_air() {
        // fast path
        return true;
    }
    if !block_state.is_collision_shape_empty() {
        return false;
    }
    let registry_block = Block::from(block_state);
    if registry_block == Block::Water {
        return false;
    }
    if block_state
        .property::<azalea_block::properties::Waterlogged>()
        .unwrap_or_default()
    {
        return false;
    }
    if registry_block == Block::Lava {
        return false;
    }
    // block.waterlogged currently doesn't account for seagrass and some other water
    // blocks
    if block_state == Block::Seagrass.into() {
        return false;
    }

    // don't walk into fire
    if registry_block == Block::Fire || registry_block == Block::SoulFire {
        return false;
    }

    if registry_block == Block::PowderSnow {
        // we can't jump out of powder snow
        return false;
    }

    if registry_block == Block::SweetBerryBush {
        // these hurt us
        return false;
    }

    true
}

/// Whether this block has a solid hitbox at the top (i.e. we can stand on it
/// and do parkour from it).
#[inline]
pub fn is_block_state_solid(block_state: BlockState) -> bool {
    if block_state.is_air() {
        // fast path
        return false;
    }
    if block_state.is_collision_shape_full() {
        return true;
    }

    if matches!(
        block_state.property::<properties::Type>(),
        Some(properties::Type::Top | properties::Type::Double)
    ) {
        // top slabs
        return true;
    }

    let block = Block::from(block_state);
    // solid enough
    if matches!(block, Block::DirtPath | Block::Farmland) {
        return true;
    }

    false
}

/// Whether we can stand on this block (but not necessarily do parkour jumps
/// from it).
pub fn is_block_state_standable(block_state: BlockState) -> bool {
    if is_block_state_solid(block_state) {
        return true;
    }

    let block = Block::from(block_state);
    if tags::blocks::SLABS.contains(&block) || tags::blocks::STAIRS.contains(&block) {
        return true;
    }

    false
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
        partial_world
            .chunks
            .set_block_state(BlockPos::new(0, 0, 0), Block::Stone.into(), &world);
        partial_world
            .chunks
            .set_block_state(BlockPos::new(0, 1, 0), BlockState::AIR, &world);

        let ctx = CachedWorld::new(Arc::new(RwLock::new(world.into())), BlockPos::default());
        assert!(!ctx.is_block_pos_passable(BlockPos::new(0, 0, 0)));
        assert!(ctx.is_block_pos_passable(BlockPos::new(0, 1, 0),));
    }

    #[test]
    fn test_is_solid() {
        let mut partial_world = PartialInstance::default();
        let mut world = ChunkStorage::default();
        partial_world
            .chunks
            .set(&ChunkPos { x: 0, z: 0 }, Some(Chunk::default()), &mut world);
        partial_world
            .chunks
            .set_block_state(BlockPos::new(0, 0, 0), Block::Stone.into(), &world);
        partial_world
            .chunks
            .set_block_state(BlockPos::new(0, 1, 0), BlockState::AIR, &world);

        let ctx = CachedWorld::new(Arc::new(RwLock::new(world.into())), BlockPos::default());
        assert!(ctx.is_block_pos_solid(BlockPos::new(0, 0, 0)));
        assert!(!ctx.is_block_pos_solid(BlockPos::new(0, 1, 0)));
    }

    #[test]
    fn test_is_standable() {
        let mut partial_world = PartialInstance::default();
        let mut world = ChunkStorage::default();
        partial_world
            .chunks
            .set(&ChunkPos { x: 0, z: 0 }, Some(Chunk::default()), &mut world);
        partial_world
            .chunks
            .set_block_state(BlockPos::new(0, 0, 0), Block::Stone.into(), &world);
        partial_world
            .chunks
            .set_block_state(BlockPos::new(0, 1, 0), BlockState::AIR, &world);
        partial_world
            .chunks
            .set_block_state(BlockPos::new(0, 2, 0), BlockState::AIR, &world);
        partial_world
            .chunks
            .set_block_state(BlockPos::new(0, 3, 0), BlockState::AIR, &world);

        let ctx = CachedWorld::new(Arc::new(RwLock::new(world.into())), BlockPos::default());
        assert!(ctx.is_standable_at_block_pos(BlockPos::new(0, 1, 0)));
        assert!(!ctx.is_standable_at_block_pos(BlockPos::new(0, 0, 0)));
        assert!(!ctx.is_standable_at_block_pos(BlockPos::new(0, 2, 0)));
    }
}
