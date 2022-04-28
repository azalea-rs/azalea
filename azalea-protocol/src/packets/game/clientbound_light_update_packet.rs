use azalea_core::{game_type::GameType, resource_location::ResourceLocation};
use packet_macros::GamePacket;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundLightUpdatePacket {
    pub x: i32,
    pub z: i32,
    pub light_data: ClientboundLightUpdatePacketData,
}

pub struct ClientboundLightUpdatePacketData {}
