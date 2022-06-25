use azalea_buf::McBuf;
use packet_macros::GamePacket;

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ServerboundMovePlayerPacketRot {
    pub y_rot: f32,
    pub x_rot: f32,
    pub on_ground: bool,
}
