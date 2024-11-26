use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundGamePacket;

use crate::packets::game::s_interact::InteractionHand;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundSwing {
    pub hand: InteractionHand,
}
