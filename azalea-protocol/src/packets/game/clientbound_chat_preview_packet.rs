use azalea_buf::McBuf;
use azalea_chat::FormattedText;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundChatPreviewPacket {
    pub query_id: i32,
    pub preview: Option<FormattedText>,
}
