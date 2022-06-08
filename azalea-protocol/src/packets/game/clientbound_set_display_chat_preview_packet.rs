use packet_macros::{GamePacket, McBuf};

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundSetDisplayChatPreviewPacket {
    pub enabled: bool,
}
