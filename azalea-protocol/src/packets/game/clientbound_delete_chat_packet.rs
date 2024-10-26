use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

use super::clientbound_player_chat_packet::PackedMessageSignature;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundDeleteChatPacket {
    pub signature: PackedMessageSignature,
}
