use azalea_buf::McBuf;
use azalea_core::position::ChunkPos;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundChunksBiomesPacket {
    pub chunk_biome_data: Vec<ChunkBiomeData>,
}

#[derive(Clone, Debug, McBuf)]
pub struct ChunkBiomeData {
    pub pos: ChunkPos,
    pub buffer: Vec<u8>,
}
