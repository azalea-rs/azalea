use azalea_buf::McBuf;
use packet_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundMovePlayerRotPacket {
    pub y_rot: f32,
    pub x_rot: f32,
    pub on_ground: bool,
}
