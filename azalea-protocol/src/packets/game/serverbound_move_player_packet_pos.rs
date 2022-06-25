use azalea_buf::McBuf;
use packet_macros::GamePacket;

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ServerboundMovePlayerPacketPos {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub on_ground: bool,
}
