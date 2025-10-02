use azalea_buf::AzBuf;
use azalea_core::position::GlobalPos;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundSetDefaultSpawnPosition {
    pub global_pos: GlobalPos,
    pub yaw: f32,
    pub pitch: f32,
}
