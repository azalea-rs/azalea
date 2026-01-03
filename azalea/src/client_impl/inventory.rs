use azalea_client::inventory::SetSelectedHotbarSlotEvent;
use azalea_entity::inventory::Inventory;
use azalea_inventory::Menu;

use crate::Client;

impl Client {
    /// Return the menu that is currently open, or the player's inventory if no
    /// menu is open.
    ///
    /// If you need to interact with the menu, consider using
    /// [`Self::open_inventory`] instead.
    pub fn menu(&self) -> Menu {
        self.component::<Inventory>().menu().clone()
    }

    /// Returns the index of the hotbar slot that's currently selected.
    ///
    /// If you want to access the actual held item, you can get the current menu
    /// with [`Client::menu`] and then get the slot index by offsetting from
    /// the start of [`azalea_inventory::Menu::hotbar_slots_range`].
    ///
    /// You can use [`Self::set_selected_hotbar_slot`] to change it.
    pub fn selected_hotbar_slot(&self) -> u8 {
        self.component::<Inventory>().selected_hotbar_slot
    }

    /// Update the selected hotbar slot index.
    ///
    /// This will run next `Update`, so you might want to call
    /// `bot.wait_updates(1)` after calling this if you're using `azalea`.
    ///
    /// # Panics
    ///
    /// This will panic if `new_hotbar_slot_index` is not in the range 0..=8.
    pub fn set_selected_hotbar_slot(&self, new_hotbar_slot_index: u8) {
        assert!(
            new_hotbar_slot_index < 9,
            "Hotbar slot index must be in the range 0..=8"
        );

        let mut ecs = self.ecs.write();
        ecs.trigger(SetSelectedHotbarSlotEvent {
            entity: self.entity,
            slot: new_hotbar_slot_index,
        });
    }
}
