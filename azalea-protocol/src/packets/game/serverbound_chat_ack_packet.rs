use crate::packets::game::clientbound_player_chat_packet::LastSeenUpdate;
use azalea_buf::McBuf;
use packet_macros::GamePacket;

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ServerboundChatAckPacket {
    pub last_seen_messages: LastSeenUpdate,
}
