use azalea_buf::AzBuf;
use azalea_core::position::Vec3;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_world::MinecraftEntityId;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundMoveMinecartAlongTrack {
    #[var]
    pub entity_id: MinecraftEntityId,
    pub lerp_steps: Vec<MinecartStep>,
}

#[derive(Clone, Debug, AzBuf, PartialEq)]
pub struct MinecartStep {
    pub position: Vec3,
    pub movement: Vec3,
    pub y_rot: u8,
    pub x_rot: u8,
    pub weight: f32,
}
