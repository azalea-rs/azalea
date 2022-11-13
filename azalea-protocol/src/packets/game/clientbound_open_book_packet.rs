use super::serverbound_interact_packet::InteractionHand;
use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundOpenBookPacket {
    pub hand: InteractionHand,
}
