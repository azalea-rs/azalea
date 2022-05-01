use azalea_core::{game_type::GameType, resource_location::ResourceLocation};
use packet_macros::GamePacket;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundLoginPacket {
    pub player_id: u32,
    pub hardcore: bool,
    pub game_type: GameType,
    pub previous_game_type: Option<GameType>,
    pub levels: Vec<ResourceLocation>,
    pub registry_holder: azalea_nbt::Tag,
    pub dimension_type: azalea_nbt::Tag,
    pub dimension: ResourceLocation,
    pub seed: i64,
    #[varint]
    pub max_players: i32,
    #[varint]
    pub chunk_radius: i32,
    #[varint]
    pub simulation_distance: i32,
    pub reduced_debug_info: bool,
    pub show_death_screen: bool,
    pub is_debug: bool,
    pub is_flat: bool,
}
