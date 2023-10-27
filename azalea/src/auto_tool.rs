use azalea_block::{Block, BlockState};
use azalea_client::{inventory::InventoryComponent, Client};
use azalea_entity::{FluidOnEyes, Physics};
use azalea_inventory::{ItemSlot, Menu};
use azalea_registry::Fluid;

pub struct BestToolResult {
    pub index: usize,
    pub percentage_per_tick: f32,
}

pub trait AutoToolClientExt {
    fn best_tool_in_hotbar_for_block(&self, block: BlockState) -> BestToolResult;
}

impl AutoToolClientExt for Client {
    fn best_tool_in_hotbar_for_block(&self, block: BlockState) -> BestToolResult {
        let mut ecs = self.ecs.lock();
        let (inventory, physics, fluid_on_eyes) =
            self.query::<(&InventoryComponent, &Physics, &FluidOnEyes)>(&mut ecs);
        let menu = &inventory.inventory_menu;

        accurate_best_tool_in_hotbar_for_block(block, menu, physics, fluid_on_eyes)
    }
}

/// Returns the best tool in the hotbar for the given block.
///
/// Note that this doesn't take into account whether the player is on the ground
/// or in water, use [`accurate_best_tool_in_hotbar_for_block`] instead if you
/// care about those things.
pub fn best_tool_in_hotbar_for_block(block: BlockState, menu: &Menu) -> BestToolResult {
    accurate_best_tool_in_hotbar_for_block(
        block,
        menu,
        &Physics {
            on_ground: true,
            velocity: Default::default(),
            xxa: Default::default(),
            yya: Default::default(),
            zza: Default::default(),
            last_on_ground: Default::default(),
            dimensions: Default::default(),
            bounding_box: Default::default(),
            has_impulse: Default::default(),
        },
        &FluidOnEyes::new(Fluid::Empty),
    )
}

pub fn accurate_best_tool_in_hotbar_for_block(
    block: BlockState,
    menu: &Menu,
    physics: &Physics,
    fluid_on_eyes: &FluidOnEyes,
) -> BestToolResult {
    let hotbar_slots = &menu.slots()[menu.hotbar_slots_range()];

    let mut best_speed = 0.;
    let mut best_slot = None;

    let block = Box::<dyn Block>::from(block);

    for (i, item_slot) in hotbar_slots.iter().enumerate() {
        if let ItemSlot::Present(item_slot) = item_slot {
            let this_item_speed = azalea_entity::mining::get_mine_progress(
                block.as_ref(),
                item_slot.kind,
                menu,
                fluid_on_eyes,
                physics,
            );
            if this_item_speed > best_speed {
                best_slot = Some(i);
                best_speed = this_item_speed;
            }
        }
    }

    BestToolResult {
        index: best_slot.unwrap_or(0),
        percentage_per_tick: best_speed,
    }
}
