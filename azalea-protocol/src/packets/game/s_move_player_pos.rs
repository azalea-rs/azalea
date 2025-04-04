use azalea_buf::AzBuf;
use azalea_core::position::Vec3;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, AzBuf, ServerboundGamePacket)]
pub struct ServerboundMovePlayerPos {
    pub pos: Vec3,
    pub on_ground: bool,
}
