use azalea_buf::AzBuf;
use azalea_core::position::ChunkPos;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundForgetLevelChunk {
    pub pos: ChunkPos,
}
