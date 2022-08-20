use azalea_buf::McBuf;
use packet_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundMovePlayerPosPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub on_ground: bool,
}
