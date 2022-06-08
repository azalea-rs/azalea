use packet_macros::{GamePacket, McBuf};

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ServerboundChatPreviewPacket {
    pub query_id: i32,
    pub query: String,
}
