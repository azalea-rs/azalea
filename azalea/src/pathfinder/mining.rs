use std::cell::UnsafeCell;

use azalea_block::BlockState;
use azalea_inventory::Menu;
use nohash_hasher::IntMap;

use crate::auto_tool::best_tool_in_hotbar_for_block;

pub struct MiningCache {
    block_state_id_costs: UnsafeCell<IntMap<u32, f32>>,
    inventory_menu: Menu,
}

impl MiningCache {
    pub fn new(inventory_menu: Menu) -> Self {
        Self {
            block_state_id_costs: UnsafeCell::new(IntMap::default()),
            inventory_menu,
        }
    }

    pub fn cost_for(&self, block: BlockState) -> f32 {
        // SAFETY: mining is single-threaded, so this is safe
        let block_state_id_costs = unsafe { &mut *self.block_state_id_costs.get() };

        if let Some(cost) = block_state_id_costs.get(&block.id) {
            *cost
        } else {
            let best_tool_result = best_tool_in_hotbar_for_block(block, &self.inventory_menu);
            let cost = 1. / best_tool_result.percentage_per_tick;
            block_state_id_costs.insert(block.id, cost);
            cost
        }
    }
}
