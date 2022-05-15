use azalea_chat::component::Component;
use packet_macros::{GamePacket, McBuf};

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundDisconnectPacket {
    pub reason: Component,
}
