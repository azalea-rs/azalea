use super::clientbound_player_chat_packet::ChatTypeBound;
use azalea_buf::McBuf;
use azalea_chat::FormattedText;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundDisguisedChatPacket {
    pub message: FormattedText,
    pub chat_type: ChatTypeBound,
}
