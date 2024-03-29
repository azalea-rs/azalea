use super::serverbound_chat_packet::LastSeenMessagesUpdate;
use azalea_buf::McBuf;
use azalea_crypto::MessageSignature;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundChatCommandSignedPacket {
    pub command: String,
    pub timestamp: u64,
    pub salt: u64,
    pub argument_signatures: Vec<ArgumentSignature>,
    pub last_seen_messages: LastSeenMessagesUpdate,
}

#[derive(Clone, Debug, McBuf)]
pub struct ArgumentSignature {
    pub name: String,
    pub signature: MessageSignature,
}
