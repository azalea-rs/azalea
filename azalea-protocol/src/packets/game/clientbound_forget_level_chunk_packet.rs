use azalea_buf::McBuf;
use azalea_core::position::ChunkPos;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundForgetLevelChunkPacket {
    pub pos: ChunkPos,
}
