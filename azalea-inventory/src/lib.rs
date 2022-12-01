use azalea_core::Slot;
use azalea_inventory_macros::declare_menus;

// the player inventory part is always the last 36 slots (except in the Player
// menu), so we don't have to explicitly specify it

// Client {
//     ...
//     pub menu: Menu,
//     pub inventory: Arc<[Slot; 36]>
// }

// Generate an `enum Menu` and `impl Menu`.
// if the `inventory` field is present, then the `player` field doesn't get
// implicitly added
declare_menus!({
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
    Chest {
        block: 27,
    }
});

impl Menu {
    /// Get a mutable reference to the [`Slot`] at the given protocol index. If
    /// you're trying to get an item in a menu normally, you should just
    /// `match` it and index the `[Slot]` you get
    pub fn slot_mut(&self, i: usize) -> Option<&Slot> {
        Some(match self {
            Menu::Player {
                craft_result,
                craft,
                armor,
                inventory,
                offhand,
            } => {
                match i {
                    0 => craft_result,
                    1..=4 => craft,
                    5..=8 => armor,
                    // ...
                    _ => return None,
                }
            } // ...
        })
    }
}
