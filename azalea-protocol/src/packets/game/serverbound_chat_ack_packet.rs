use crate::packets::game::clientbound_player_chat_packet::LastSeenMessagesUpdate;
use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundChatAckPacket {
    pub last_seen_messages: LastSeenMessagesUpdate,
}
