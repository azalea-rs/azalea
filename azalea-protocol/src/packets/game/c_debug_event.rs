use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

use crate::common::debug_subscription::DebugSubscriptionEvent;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundDebugEvent {
    pub event: DebugSubscriptionEvent,
}
