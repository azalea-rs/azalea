use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_world::MinecraftEntityId;

use crate::common::movements::{PositionMoveRotation, RelativeMovements};

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundTeleportEntity {
    #[var]
    pub id: MinecraftEntityId,
    pub change: PositionMoveRotation,
    pub relative: RelativeMovements,
    pub on_ground: bool,
}
