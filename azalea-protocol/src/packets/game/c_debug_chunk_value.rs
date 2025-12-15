use azalea_buf::AzBuf;
use azalea_core::position::ChunkPos;
use azalea_protocol_macros::ClientboundGamePacket;

use crate::common::debug_subscription::DebugSubscriptionUpdate;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundDebugChunkValue {
    pub chunk_pos: ChunkPos,
    pub update: DebugSubscriptionUpdate,
}
