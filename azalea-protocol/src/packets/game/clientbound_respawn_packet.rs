use azalea_buf::McBuf;
use azalea_core::{GameMode, GlobalPos, OptionalGameType, ResourceLocation};
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundRespawnPacket {
    pub dimension_type: ResourceLocation,
    pub dimension: ResourceLocation,
    pub seed: u64,
    pub player_game_type: GameMode,
    pub previous_player_game_type: OptionalGameType,
    pub is_debug: bool,
    pub is_flat: bool,
    pub data_to_keep: u8,
    pub last_death_location: Option<GlobalPos>,
    #[var]
    pub portal_cooldown: u32,
}
