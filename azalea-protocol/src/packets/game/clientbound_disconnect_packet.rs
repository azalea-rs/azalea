use azalea_chat::component::Component;
use packet_macros::GamePacket;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundDisconnectPacket {
    pub reason: Component,
}
