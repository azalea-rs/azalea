use azalea_chat::component::Component;
use packet_macros::LoginPacket;

#[derive(Clone, Debug, LoginPacket)]
pub struct ClientboundLoginDisconnectPacket {
    pub reason: Component,
}
