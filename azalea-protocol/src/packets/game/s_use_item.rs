use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;

use crate::packets::game::s_interact::InteractionHand;

#[derive(Clone, Debug, AzBuf, PartialEq, ServerboundGamePacket)]
pub struct ServerboundUseItem {
    pub hand: InteractionHand,
    #[var]
    pub seq: u32,
    pub y_rot: f32,
    pub x_rot: f32,
}
