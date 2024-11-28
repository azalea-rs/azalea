use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

use super::{c_entity_position_sync::PositionMoveRotation, c_player_position::RelativeMovements};

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundRecipeBookRemove {
    #[var]
    pub id: u32,
    pub change: PositionMoveRotation,
    pub relatives: RelativeMovements,
}
