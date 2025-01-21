use azalea_buf::AzBuf;
use azalea_core::{
    game_type::{GameMode, OptionalGameType},
    position::GlobalPos,
    resource_location::ResourceLocation,
};

#[derive(Clone, Debug, AzBuf)]
pub struct CommonPlayerSpawnInfo {
    pub dimension_type: azalea_registry::DimensionType,
    pub dimension: ResourceLocation,
    pub seed: i64,
    pub game_type: GameMode,
    pub previous_game_type: OptionalGameType,
    pub is_debug: bool,
    pub is_flat: bool,
    pub last_death_location: Option<GlobalPos>,
    #[var]
    pub portal_cooldown: u32,
    #[var]
    pub sea_level: i32,
}
