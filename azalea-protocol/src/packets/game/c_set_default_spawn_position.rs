use azalea_buf::McBuf;
use azalea_core::position::BlockPos;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSetDefaultSpawnPosition {
    pub pos: BlockPos,
    pub angle: f32,
}
