use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

use super::s_interact::InteractionHand;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundOpenBook {
    pub hand: InteractionHand,
}
