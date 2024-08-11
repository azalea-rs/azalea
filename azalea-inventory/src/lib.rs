#![feature(trait_upcasting)]

/// Representations of various inventory data structures in Minecraft.
pub mod components;
pub mod item;
pub mod operations;
mod slot;

use std::ops::{Deref, DerefMut, RangeInclusive};

use azalea_inventory_macros::declare_menus;
pub use slot::{DataComponentPatch, ItemSlot, ItemSlotData};

// TODO: remove this here and in azalea-inventory-macros when rust makes
// Default be implemented for all array sizes
// https://github.com/rust-lang/rust/issues/61415

/// A fixed-size list of [`ItemSlot`]s.
#[derive(Debug, Clone)]
pub struct SlotList<const N: usize>([ItemSlot; N]);
impl<const N: usize> Deref for SlotList<N> {
    type Target = [ItemSlot; N];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<const N: usize> DerefMut for SlotList<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<const N: usize> Default for SlotList<N> {
    fn default() -> Self {
        SlotList([(); N].map(|_| ItemSlot::Empty))
    }
}

impl Menu {
    /// Get the [`Player`] from this [`Menu`].
    ///
    /// # Panics
    ///
    /// Will panic if the menu isn't `Menu::Player`.
    pub fn as_player(&self) -> &Player {
        if let Menu::Player(player) = &self {
            player
        } else {
            unreachable!("Called `Menu::as_player` on a menu that wasn't `Player`.")
        }
    }
}

// the player inventory part is always the last 36 slots (except in the Player
// menu), so we don't have to explicitly specify it

// Client {
//     ...
//     pub menu: Menu,
//     pub inventory: Arc<[Slot; 36]>
// }

// Generate a `struct Player`, `enum Menu`, and `impl Menu`.
// a "player" field gets implicitly added with the player inventory

declare_menus! {
    Player {
        craft_result: 1,
        craft: 4,
        armor: 4,
        inventory: 36,
        offhand: 1,
    },
    Generic9x1 {
        contents: 9,
    },
    Generic9x2 {
        contents: 18,
    },
    Generic9x3 {
        contents: 27,
    },
    Generic9x4 {
        contents: 36,
    },
    Generic9x5 {
        contents: 45,
    },
    Generic9x6 {
        contents: 54,
    },
    Generic3x3 {
        contents: 9,
    },
    Crafter3x3 {
        contents: 9,
    },
    Anvil {
        first: 1,
        second: 1,
        result: 1,
    },
    Beacon {
        payment: 1,
    },
    BlastFurnace {
        ingredient: 1,
        fuel: 1,
        result: 1,
    },
    BrewingStand {
        bottles: 3,
        ingredient: 1,
        fuel: 1,
    },
    Crafting {
        result: 1,
        grid: 9,
    },
    Enchantment {
        item: 1,
        lapis: 1,
    },
    Furnace {
        ingredient: 1,
        fuel: 1,
        result: 1,
    },
    Grindstone {
        input: 1,
        additional: 1,
        result: 1,
    },
    Hopper {
        contents: 5,
    },
    Lectern {
        book: 1,
    },
    Loom {
        banner: 1,
        dye: 1,
        pattern: 1,
        result: 1,
    },
    Merchant {
        payments: 2,
        result: 1,
    },
    ShulkerBox {
        contents: 27,
    },
    Smithing {
        template: 1,
        base: 1,
        additional: 1,
        result: 1,
    },
    Smoker {
        ingredient: 1,
        fuel: 1,
        result: 1,
    },
    CartographyTable {
        map: 1,
        additional: 1,
        result: 1,
    },
    Stonecutter {
        input: 1,
        result: 1,
    },
}
