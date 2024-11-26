use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

use super::s_interact::InteractionHand;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundOpenBook {
    pub hand: InteractionHand,
}
