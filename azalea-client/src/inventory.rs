use std::fmt;

use azalea_ecs::component::Component;
use azalea_inventory::Menu;
use derive_more::{Deref, DerefMut};

/// A component that contains the player's inventory menu. This is guaranteed to
/// be a `Menu::Player`.
#[derive(Component, Deref, DerefMut)]
pub struct InventoryMenu(azalea_inventory::Menu);
impl Default for InventoryMenu {
    fn default() -> Self {
        InventoryMenu(Menu::Player(azalea_inventory::Player::default()))
    }
}
impl fmt::Debug for InventoryMenu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Menu::Player(player) = &self.0 {
            f.debug_tuple("InventoryMenu").field(&player).finish()
        } else {
            unreachable!()
        }
    }
}

/// A component that contains information about the container that's currently
/// open. Only present if a container is open.
#[derive(Component)]
pub struct OpenContainerMenu {
    /// The ID of the container that's currently open. Its value is not
    /// guaranteed to be anything specific, and may change every time you open a
    /// container.
    ///
    /// This variable technically won't ever be 0, since if it's 0 that means
    /// there's no container open and this component won't be present.
    pub id: u8,
    pub menu: azalea_inventory::Menu,
}
