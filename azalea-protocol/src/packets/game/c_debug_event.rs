use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

use crate::common::debug_subscription::DebugSubscriptionEvent;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundDebugEvent {
    pub event: DebugSubscriptionEvent,
}
