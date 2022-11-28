use std::sync::Arc;

use azalea_core::Slot;

// the player inventory part is always the last 36 slots (except in the Player
// menu), so we don't have to explicitly specify it

// if "inventory" is present, then the `player` inventory part doesn't get
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

pub enum Menu {
    Player {
        craft_result: Slot,
        craft: [Slot; 4],
        armor: [Slot; 4],
        inventory: [Slot; 36],
        offhand: Slot,
    },
    Generic9x1 {
        contents: Slots<9>,
        player: Arc<[Slot; 36]>,
    },
    Generic9x2 {
        contents: Slots<18>,
        player: Arc<[Slot; 36]>,
    },
    Generic9x3 {
        contents: [Slot; 36],
        player: Arc<[Slot; 36]>,
    },
}
impl Menu {}
