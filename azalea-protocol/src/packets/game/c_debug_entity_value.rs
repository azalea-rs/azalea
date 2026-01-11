use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_core::entity_id::MinecraftEntityId;

use crate::common::debug_subscription::DebugSubscriptionUpdate;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundDebugEntityValue {
    pub entity_id: MinecraftEntityId,
    pub update: DebugSubscriptionUpdate,
}
