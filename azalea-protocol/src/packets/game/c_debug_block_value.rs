use azalea_buf::AzBuf;
use azalea_core::position::BlockPos;
use azalea_protocol_macros::ClientboundGamePacket;

use crate::common::debug_subscription::DebugSubscriptionUpdate;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundDebugBlockValue {
    pub block_pos: BlockPos,
    pub update: DebugSubscriptionUpdate,
}
