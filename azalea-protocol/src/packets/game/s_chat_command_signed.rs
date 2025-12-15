use azalea_buf::AzBuf;
use azalea_crypto::signing::MessageSignature;
use azalea_protocol_macros::ServerboundGamePacket;

use super::s_chat::LastSeenMessagesUpdate;

#[derive(AzBuf, Clone, Debug, PartialEq, ServerboundGamePacket)]
pub struct ServerboundChatCommandSigned {
    pub command: String,
    pub timestamp: u64,
    pub salt: u64,
    pub argument_signatures: Vec<ArgumentSignature>,
    pub last_seen_messages: LastSeenMessagesUpdate,
}

#[derive(AzBuf, Clone, Debug, PartialEq)]
pub struct ArgumentSignature {
    pub name: String,
    pub signature: MessageSignature,
}
