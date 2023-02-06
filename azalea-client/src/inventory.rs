use std::fmt;

use azalea_core::{Slot, SlotData};
use azalea_ecs::component::Component;
use azalea_inventory::Menu;
use derive_more::{Deref, DerefMut};

/// A component that contains the player's inventory menu. This is guaranteed to
/// be a `Menu::Player`.
///
/// We keep it as a [`Menu`] since `Menu` has some useful functions that bare
/// [`azalea_inventory::Player`] doesn't have.
#[derive(Component, Deref, DerefMut)]
pub struct InventoryMenu(azalea_inventory::Menu);
impl Default for InventoryMenu {
    fn default() -> Self {
        InventoryMenu(Menu::Player(azalea_inventory::Player::default()))
    }
}
impl InventoryMenu {
    pub fn as_player(self) -> &azalea_inventory::Player {
        if let Menu::Player(player) = &self.0 {
            player
        } else {
            unreachable!("InventoryMenu must always be a Menu::Player")
        }
    }
}
impl fmt::Debug for InventoryMenu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("InventoryMenu")
            .field(&self.as_player())
            .finish()
    }
}
impl Clone for InventoryMenu {
    fn clone(&self) -> Self {
        InventoryMenu(Menu::Player(self.as_player().clone()))
    }
}

/// A component that contains information about the container that's currently
/// open.
/// It will be present on all players that can have a container open.
#[derive(Component)]
pub struct ActiveContainer {
    /// The ID of the container that's currently open. Its value is not
    /// guaranteed to be anything specific, and may change every time you open a
    /// container (unless it's 0, in which case it means that no container is
    /// open).
    pub id: u8,
    pub menu: azalea_inventory::Menu,

    /// The item that is currently held by the cursor. `Slot::Empty` if nothing
    /// is currently being held.
    pub carried: Slot,
    pub state_id: u32,
    // minecraft also has these fields, but i don't need they're necessary?:
    // private final NonNullList<ItemStack> remoteSlots;
    // private final IntList remoteDataSlots;
    // private ItemStack remoteCarried;
}
impl Default for ActiveContainer {
    fn default() -> Self {
        ActiveContainer {
            id: 0,
            menu: Menu::Player(azalea_inventory::Player::default()),
            carried: Slot::Empty,
            state_id: 0,
        }
    }
}
