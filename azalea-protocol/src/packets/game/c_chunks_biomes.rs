use azalea_buf::AzBuf;
use azalea_core::position::ChunkPos;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundChunksBiomes {
    pub chunk_biome_data: Vec<ChunkBiomeData>,
}

#[derive(Clone, Debug, AzBuf)]
pub struct ChunkBiomeData {
    pub pos: ChunkPos,
    pub buffer: Vec<u8>,
}
