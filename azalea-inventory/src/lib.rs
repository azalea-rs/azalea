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
        input: 1,
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
