use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_world::MinecraftEntityId;

use crate::common::movements::PositionMoveRotation;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundEntityPositionSync {
    #[var]
    pub id: MinecraftEntityId,
    pub values: PositionMoveRotation,
    pub on_ground: bool,
}
