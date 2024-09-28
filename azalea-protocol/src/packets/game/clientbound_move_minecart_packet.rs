use azalea_buf::McBuf;
use azalea_core::position::Vec3;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundMoveMinecartPacket {
    #[var]
    pub entity_id: u32,
    pub lerp_steps: Vec<MinecartStep>,
}

#[derive(Clone, Debug, McBuf)]
pub struct MinecartStep {
    pub position: Vec3,
    pub movement: Vec3,
    pub y_rot: u8,
    pub x_rot: u8,
    pub weight: f32,
}
