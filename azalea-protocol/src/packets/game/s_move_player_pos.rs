use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, AzBuf, ServerboundGamePacket)]
pub struct ServerboundMovePlayerPos {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub on_ground: bool,
}
