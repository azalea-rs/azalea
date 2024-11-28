use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;

use crate::packets::game::s_interact::InteractionHand;

#[derive(Clone, Debug, AzBuf, ServerboundGamePacket)]
pub struct ServerboundSwing {
    pub hand: InteractionHand,
}
