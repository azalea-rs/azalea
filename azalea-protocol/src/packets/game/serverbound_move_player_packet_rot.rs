use packet_macros::{GamePacket, McBuf};

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ServerboundMovePlayerPacketRot {
    pub y_rot: f32,
    pub x_rot: f32,
    pub on_ground: bool,
}
