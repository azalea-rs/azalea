use azalea_block::BlockState;
use azalea_inventory::Menu;
use nohash_hasher::IntMap;

use crate::auto_tool::best_tool_in_hotbar_for_block;

pub struct MiningCache {
    block_state_id_costs: IntMap<u32, f32>,
    inventory_menu: Menu,
}

impl MiningCache {
    pub fn new(inventory_menu: Menu) -> Self {
        Self {
            block_state_id_costs: IntMap::default(),
            inventory_menu,
        }
    }

    pub fn cost_for(&mut self, block: BlockState) -> f32 {
        if let Some(cost) = self.block_state_id_costs.get(&block.id) {
            *cost
        } else {
            let best_tool_result = best_tool_in_hotbar_for_block(block, &self.inventory_menu);
            let cost = 1. / best_tool_result.percentage_per_tick;
            self.block_state_id_costs.insert(block.id, cost);
            cost
        }
    }
}
