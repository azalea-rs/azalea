use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundGamePacket;

use crate::packets::game::serverbound_interact_packet::InteractionHand;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundSwingPacket {
    pub hand: InteractionHand,
}
