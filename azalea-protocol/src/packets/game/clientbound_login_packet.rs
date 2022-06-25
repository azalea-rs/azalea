use azalea_buf::McBuf;
use azalea_core::{GameType, GlobalPos, OptionalGameType, ResourceLocation};
use packet_macros::GamePacket;

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundLoginPacket {
    pub player_id: u32,
    pub hardcore: bool,
    pub game_type: GameType,
    pub previous_game_type: OptionalGameType,
    pub levels: Vec<ResourceLocation>,
    pub registry_holder: azalea_nbt::Tag,
    pub dimension_type: ResourceLocation,
    pub dimension: ResourceLocation,
    pub seed: i64,
    #[var]
    pub max_players: i32,
    #[var]
    pub chunk_radius: i32,
    #[var]
    pub simulation_distance: i32,
    pub reduced_debug_info: bool,
    pub show_death_screen: bool,
    pub is_debug: bool,
    pub is_flat: bool,
    pub last_death_location: Option<GlobalPos>,
}
