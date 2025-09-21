use azalea_buf::AzBuf;
use azalea_core::position::Vec3;
use azalea_protocol_macros::ServerboundGamePacket;

use crate::common::movements::MoveFlags;

#[derive(Clone, Debug, AzBuf, PartialEq, ServerboundGamePacket)]
pub struct ServerboundMovePlayerPos {
    pub pos: Vec3,
    pub flags: MoveFlags,
}
