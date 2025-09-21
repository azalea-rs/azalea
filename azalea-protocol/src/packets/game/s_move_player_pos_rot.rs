use azalea_buf::AzBuf;
use azalea_core::position::Vec3;
use azalea_entity::LookDirection;
use azalea_protocol_macros::ServerboundGamePacket;

use crate::common::movements::MoveFlags;

#[derive(Clone, Debug, AzBuf, PartialEq, ServerboundGamePacket)]
pub struct ServerboundMovePlayerPosRot {
    pub pos: Vec3,
    pub look_direction: LookDirection,
    pub flags: MoveFlags,
}
