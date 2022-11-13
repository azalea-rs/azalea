use azalea_buf::McBuf;
use azalea_crypto::MessageSignature;
use azalea_protocol_macros::ServerboundGamePacket;

use super::clientbound_player_chat_packet::LastSeenMessagesUpdate;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundChatCommandPacket {
    pub command: String,
    // TODO: Choose a real timestamp type
    pub timestamp: u64,
    pub salt: u64,
    pub argument_signatures: Vec<ArgumentSignature>,
    pub signed_preview: bool,
    pub last_seen_messages: LastSeenMessagesUpdate,
}

#[derive(Clone, Debug, McBuf)]
pub struct ArgumentSignature {
    pub name: String,
    pub signature: MessageSignature,
}
