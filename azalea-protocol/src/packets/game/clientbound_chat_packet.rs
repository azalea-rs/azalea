use azalea_buf::McBuf;
use azalea_chat::FormattedText;
use azalea_protocol_macros::ClientboundGamePacket;
use uuid::Uuid;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundChatPacket {
    pub message: FormattedText,
    pub kind: u8, // TODO: Does ChatType::getIndex, may not be implemented
    pub sender: Uuid,
}
