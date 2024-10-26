use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

use super::serverbound_interact_packet::InteractionHand;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundOpenBookPacket {
    pub hand: InteractionHand,
}
