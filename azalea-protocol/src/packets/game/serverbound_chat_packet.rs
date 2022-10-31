use azalea_buf::McBuf;
use azalea_core::FixedBitSet;
use azalea_crypto::MessageSignature;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundChatPacket {
    pub message: String,
    pub timestamp: u64,
    pub salt: u64,
    pub signature: Option<MessageSignature>,
    pub last_seen_messages: LastSeenMessagesUpdate,
}

#[derive(Clone, Debug, McBuf, Default)]
pub struct LastSeenMessagesUpdate {
    #[var]
    pub offset: u32,
    pub acknowledged: FixedBitSet<20>,
}
