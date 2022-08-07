use crate::packets::game::serverbound_interact_packet::InteractionHand;
use azalea_buf::McBuf;
use packet_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundSwingPacket {
    pub hand: InteractionHand,
}
