use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

use super::{
    clientbound_entity_position_sync_packet::PositionMoveRotation,
    clientbound_player_position_packet::RelativeMovements,
};

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundRecipeBookRemovePacket {
    #[var]
    pub id: u32,
    pub change: PositionMoveRotation,
    pub relatives: RelativeMovements,
}
