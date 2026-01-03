use azalea_buf::AzBuf;
use azalea_core::position::GlobalPos;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundSetDefaultSpawnPosition {
    pub global_pos: GlobalPos,
    pub yaw: f32,
    pub pitch: f32,
}
