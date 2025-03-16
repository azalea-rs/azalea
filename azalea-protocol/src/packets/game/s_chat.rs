use azalea_buf::AzBuf;
use azalea_core::bitset::FixedBitSet;
use azalea_crypto::MessageSignature;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, AzBuf, ServerboundGamePacket)]
pub struct ServerboundChat {
    #[limit(256)]
    pub message: String,
    pub timestamp: u64,
    pub salt: u64,
    pub signature: Option<MessageSignature>,
    pub last_seen_messages: LastSeenMessagesUpdate,
}

#[derive(Clone, Debug, AzBuf, Default)]
pub struct LastSeenMessagesUpdate {
    #[var]
    pub offset: u32,
    pub acknowledged: FixedBitSet<{ 20_usize.div_ceil(8) }>,
    pub checksum: u8,
}
