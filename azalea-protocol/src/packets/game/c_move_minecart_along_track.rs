use azalea_buf::AzBuf;
use azalea_core::position::Vec3;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_world::MinecraftEntityId;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundMoveMinecartAlongTrack {
    #[var]
    pub entity_id: MinecraftEntityId,
    pub lerp_steps: Vec<MinecartStep>,
}

#[derive(AzBuf, Clone, Debug, PartialEq)]
pub struct MinecartStep {
    pub position: Vec3,
    pub movement: Vec3,
    pub y_rot: u8,
    pub x_rot: u8,
    pub weight: f32,
}
