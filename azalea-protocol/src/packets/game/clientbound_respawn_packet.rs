use azalea_buf::McBuf;
use azalea_core::{GameType, GlobalPos, OptionalGameType, ResourceLocation};
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundRespawnPacket {
    pub dimension_type: ResourceLocation,
    pub dimension: ResourceLocation,
    pub seed: u64,
    pub player_game_type: GameType,
    pub previous_player_game_type: OptionalGameType,
    pub is_debug: bool,
    pub is_flat: bool,
    pub keep_all_player_data: bool,
    pub last_death_location: Option<GlobalPos>,
}
