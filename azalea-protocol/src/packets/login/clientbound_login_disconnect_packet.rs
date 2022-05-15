use azalea_chat::component::Component;
use packet_macros::{LoginPacket, McBuf};

#[derive(Clone, Debug, McBuf, LoginPacket)]
pub struct ClientboundLoginDisconnectPacket {
    pub reason: Component,
}
