use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundMovePlayerRot {
    pub y_rot: f32,
    pub x_rot: f32,
    pub on_ground: bool,
}
