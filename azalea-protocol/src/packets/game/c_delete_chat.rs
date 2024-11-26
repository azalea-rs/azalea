use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

use super::c_player_chat::PackedMessageSignature;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundDeleteChat {
    pub signature: PackedMessageSignature,
}
