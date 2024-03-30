// these are definitely still registries, but they're not provided by mojang's
// data generator

use crate::Registry;
use azalea_registry_macros::registry;

registry! {
enum WolfVariant {
    Pale => "minecraft:wolf",
    Spotted => "minecraft:wolf_spotted",
    Snowy => "minecraft:wolf_snowy",
    Black => "minecraft:wolf_black",
    Ashen => "minecraft:wolf_ashen",
    Rusty => "minecraft:wolf_rusty",
    Woods => "minecraft:wolf_woods",
    Chestnut => "minecraft:wolf_chestnut",
    Striped => "minecraft:wolf_striped",
}
}

impl Default for WolfVariant {
    fn default() -> Self {
        WolfVariant::Pale
    }
}
