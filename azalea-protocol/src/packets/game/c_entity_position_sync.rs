use azalea_buf::AzBuf;
use azalea_core::position::Vec3;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundEntityPositionSync {
    #[var]
    pub id: u32,
    pub values: PositionMoveRotation,
    pub on_ground: bool,
}

#[derive(AzBuf, Clone, Debug)]
pub struct PositionMoveRotation {
    pub position: Vec3,
    pub delta_movement: Vec3,
    pub y_rot: f32,
    pub x_rot: f32,
}
