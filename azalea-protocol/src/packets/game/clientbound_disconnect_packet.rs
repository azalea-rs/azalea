use azalea_chat::component::Component;
use azalea_core::resource_location::ResourceLocation;
use packet_macros::GamePacket;
use serde::Deserialize;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundDisconnectPacket {
    pub reason: Component,
}
