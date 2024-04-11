//! These registries are sent by the server during the configuration state so
//! you should be relying on those if possible, but these are provided for your
//! convenience anyways.

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

registry! {
enum DimensionType {
    Overworld => "minecraft:overworld",
    Nether => "minecraft:the_nether",
    End => "minecraft:the_end",
    OverworldCaves => "minecraft:overworld_caves",
}
}
