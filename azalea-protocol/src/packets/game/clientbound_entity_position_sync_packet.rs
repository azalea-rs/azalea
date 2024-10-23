use azalea_buf::McBuf;
use azalea_core::position::Vec3;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundEntityPositionSyncPacket {
    #[var]
    pub id: u32,
    pub values: PositionMoveRotation,
    pub on_ground: bool,
}

#[derive(McBuf, Clone, Debug)]
pub struct PositionMoveRotation {
    pub position: Vec3,
    pub delta_movement: Vec3,
    pub y_rot: f32,
    pub x_rot: f32,
}
