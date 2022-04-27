use azalea_core::{game_type::GameType, resource_location::ResourceLocation};
use packet_macros::GamePacket;

// we can't identify the status in azalea-protocol since they vary depending on the entity
#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundEntityEventPacket {
    pub entity_id: i32,
    pub entity_status: i8,
}
