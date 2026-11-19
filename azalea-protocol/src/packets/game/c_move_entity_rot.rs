use azalea_buf::AzBuf;
use azalea_core::entity_id::MinecraftEntityId;
use azalea_protocol_macros::ClientboundGamePacket;

use crate::packets::game::c_move_entity_pos_rot::CompactLookDirection;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundMoveEntityRot {
    #[var]
    pub entity_id: MinecraftEntityId,
    pub on_ground: bool,
    pub look_direction: CompactLookDirection,
}

// this doesn't use the Properties from c_move_entity_pos_rot because that just
// provides the step count, which we don't need if we don't have movement
