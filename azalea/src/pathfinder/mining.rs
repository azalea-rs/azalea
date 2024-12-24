use std::{cell::UnsafeCell, ops::RangeInclusive};

use azalea_block::{properties::Waterlogged, BlockState, BlockStateIntegerRepr, BlockStates};
use azalea_inventory::Menu;
use nohash_hasher::IntMap;

use super::costs::BLOCK_BREAK_ADDITIONAL_PENALTY;
use crate::auto_tool::best_tool_in_hotbar_for_block;

pub struct MiningCache {
    block_state_id_costs: UnsafeCell<IntMap<BlockStateIntegerRepr, f32>>,
    inventory_menu: Option<Menu>,

    water_block_state_range: RangeInclusive<BlockStateIntegerRepr>,
    lava_block_state_range: RangeInclusive<BlockStateIntegerRepr>,

    falling_blocks: Vec<BlockState>,
}

impl MiningCache {
    pub fn new(inventory_menu: Option<Menu>) -> Self {
        let water_block_states = BlockStates::from(azalea_registry::Block::Water);
        let lava_block_states = BlockStates::from(azalea_registry::Block::Lava);

        let mut water_block_state_range_min = BlockStateIntegerRepr::MAX;
        let mut water_block_state_range_max = BlockStateIntegerRepr::MIN;
        for state in water_block_states {
            water_block_state_range_min = water_block_state_range_min.min(state.id);
            water_block_state_range_max = water_block_state_range_max.max(state.id);
        }
        let water_block_state_range = water_block_state_range_min..=water_block_state_range_max;

        let mut lava_block_state_range_min = BlockStateIntegerRepr::MAX;
        let mut lava_block_state_range_max = BlockStateIntegerRepr::MIN;
        for state in lava_block_states {
            lava_block_state_range_min = lava_block_state_range_min.min(state.id);
            lava_block_state_range_max = lava_block_state_range_max.max(state.id);
        }
        let lava_block_state_range = lava_block_state_range_min..=lava_block_state_range_max;

        let mut falling_blocks: Vec<BlockState> = vec![
            azalea_registry::Block::Sand.into(),
            azalea_registry::Block::RedSand.into(),
            azalea_registry::Block::Gravel.into(),
            azalea_registry::Block::Anvil.into(),
            azalea_registry::Block::ChippedAnvil.into(),
            azalea_registry::Block::DamagedAnvil.into(),
            // concrete powders
            azalea_registry::Block::WhiteConcretePowder.into(),
            azalea_registry::Block::OrangeConcretePowder.into(),
            azalea_registry::Block::MagentaConcretePowder.into(),
            azalea_registry::Block::LightBlueConcretePowder.into(),
            azalea_registry::Block::YellowConcretePowder.into(),
            azalea_registry::Block::LimeConcretePowder.into(),
            azalea_registry::Block::PinkConcretePowder.into(),
            azalea_registry::Block::GrayConcretePowder.into(),
            azalea_registry::Block::LightGrayConcretePowder.into(),
            azalea_registry::Block::CyanConcretePowder.into(),
            azalea_registry::Block::PurpleConcretePowder.into(),
            azalea_registry::Block::BlueConcretePowder.into(),
            azalea_registry::Block::BrownConcretePowder.into(),
            azalea_registry::Block::GreenConcretePowder.into(),
            azalea_registry::Block::RedConcretePowder.into(),
            azalea_registry::Block::BlackConcretePowder.into(),
        ];
        falling_blocks.sort_unstable_by_key(|block| block.id);

        Self {
            block_state_id_costs: UnsafeCell::new(IntMap::default()),
            inventory_menu,
            water_block_state_range,
            lava_block_state_range,
            falling_blocks,
        }
    }

    pub fn cost_for(&self, block: BlockState) -> f32 {
        let Some(inventory_menu) = &self.inventory_menu else {
            return f32::INFINITY;
        };

        // SAFETY: mining is single-threaded, so this is safe
        let block_state_id_costs = unsafe { &mut *self.block_state_id_costs.get() };

        if let Some(cost) = block_state_id_costs.get(&block.id) {
            *cost
        } else {
            let best_tool_result = best_tool_in_hotbar_for_block(block, inventory_menu);
            let mut cost = 1. / best_tool_result.percentage_per_tick;

            cost += BLOCK_BREAK_ADDITIONAL_PENALTY;

            block_state_id_costs.insert(block.id, cost);
            cost
        }
    }

    pub fn is_liquid(&self, block: BlockState) -> bool {
        // this already runs in about 1 nanosecond, so if you wanna try optimizing it at
        // least run the benchmarks (in benches/checks.rs)

        self.water_block_state_range.contains(&block.id)
            || self.lava_block_state_range.contains(&block.id)
            || is_waterlogged(block)
    }

    pub fn is_falling_block(&self, block: BlockState) -> bool {
        self.falling_blocks
            .binary_search_by_key(&block.id, |block| block.id)
            .is_ok()
    }
}

pub fn is_waterlogged(block: BlockState) -> bool {
    block.property::<Waterlogged>().unwrap_or_default()
}
