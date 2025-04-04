use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

use super::c_player_chat::PackedMessageSignature;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundDeleteChat {
    pub signature: PackedMessageSignature,
}
