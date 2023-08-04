use azalea_buf::McBuf;
use azalea_core::{GameMode, GlobalPos, OptionalGameType, ResourceLocation};

#[derive(Clone, Debug, McBuf)]
pub struct CommonPlayerSpawnInfo {
    pub dimension_type: ResourceLocation,
    pub dimension: ResourceLocation,
    pub seed: i64,
    pub game_type: GameMode,
    pub previous_game_type: OptionalGameType,
    pub is_debug: bool,
    pub is_flat: bool,
    pub last_death_location: Option<GlobalPos>,
    #[var]
    pub portal_cooldown: u32,
}
