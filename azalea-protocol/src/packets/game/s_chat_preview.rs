use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundChatPreview {
    pub query_id: i32,
    pub query: String,
}
