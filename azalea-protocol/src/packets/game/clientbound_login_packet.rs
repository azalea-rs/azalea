use crate::packets::common::CommonPlayerSpawnInfo;

use azalea_buf::McBuf;
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ClientboundGamePacket;

/// The first packet sent by the server to the client after login.
///
/// This packet contains information about the state of the player, the
/// world, and the registry.
#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundLoginPacket {
    pub player_id: u32,
    pub hardcore: bool,
    pub levels: Vec<ResourceLocation>,
    #[var]
    pub max_players: i32,
    #[var]
    pub chunk_radius: u32,
    #[var]
    pub simulation_distance: u32,
    pub reduced_debug_info: bool,
    pub show_death_screen: bool,
    pub do_limited_crafting: bool,
    pub common: CommonPlayerSpawnInfo,
}
