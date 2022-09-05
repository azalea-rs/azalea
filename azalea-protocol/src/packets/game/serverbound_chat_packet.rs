use crate::packets::game::clientbound_player_chat_packet::LastSeenMessagesUpdate;
use azalea_buf::McBuf;
use azalea_crypto::MessageSignature;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundChatPacket {
    pub message: String,
    pub timestamp: u64,
    pub salt: u64,
    pub signature: MessageSignature,
    pub signed_preview: bool,
    pub last_seen_messages: LastSeenMessagesUpdate,
}
