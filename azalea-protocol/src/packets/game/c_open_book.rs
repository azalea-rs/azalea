use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

use super::s_interact::InteractionHand;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundOpenBook {
    pub hand: InteractionHand,
}
