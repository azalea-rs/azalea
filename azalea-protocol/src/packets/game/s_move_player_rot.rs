use azalea_buf::AzBuf;
use azalea_entity::LookDirection;
use azalea_protocol_macros::ServerboundGamePacket;

use crate::common::movements::MoveFlags;

#[derive(AzBuf, Clone, Debug, PartialEq, ServerboundGamePacket)]
pub struct ServerboundMovePlayerRot {
    pub look_direction: LookDirection,
    pub flags: MoveFlags,
}
