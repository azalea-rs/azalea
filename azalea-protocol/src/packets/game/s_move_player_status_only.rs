use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;

use crate::common::movements::MoveFlags;

#[derive(Clone, Debug, AzBuf, PartialEq, ServerboundGamePacket)]
pub struct ServerboundMovePlayerStatusOnly {
    pub flags: MoveFlags,
}
