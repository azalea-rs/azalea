use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;

use crate::packets::game::s_interact::InteractionHand;

#[derive(Clone, Debug, AzBuf, ServerboundGamePacket)]
pub struct ServerboundUseItem {
    pub hand: InteractionHand,
    #[var]
    pub sequence: u32,
    pub yaw: f32,
    pub pitch: f32,
}
