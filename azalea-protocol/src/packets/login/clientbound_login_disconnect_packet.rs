use azalea_buf::McBuf;
use azalea_chat::component::Component;
use packet_macros::ClientboundLoginPacket;

#[derive(Clone, Debug, McBuf, ClientboundLoginPacket)]
pub struct ClientboundLoginDisconnectPacket {
    pub reason: Component,
}
