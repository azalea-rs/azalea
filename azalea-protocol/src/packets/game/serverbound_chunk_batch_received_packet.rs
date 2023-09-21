use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundChunkBatchReceivedPacket {
    pub desired_chunks_per_tick: f32,
}
