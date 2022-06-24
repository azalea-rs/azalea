use azalea_buf::McBuf;
use azalea_chat::component::Component;
use packet_macros::LoginPacket;

#[derive(Clone, Debug, McBuf, LoginPacket)]
pub struct ClientboundLoginDisconnectPacket {
    pub reason: Component,
}
