use azalea_buf::McBuf;
use azalea_crypto::MessageSignature;
use azalea_protocol_macros::ServerboundGamePacket;

use super::s_chat::LastSeenMessagesUpdate;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundChatCommandSigned {
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
