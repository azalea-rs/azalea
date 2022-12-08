use super::clientbound_player_chat_packet::PackedMessageSignature;
use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundDeleteChatPacket {
    pub signature: PackedMessageSignature,
}
