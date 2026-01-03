use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;
use azalea_registry::builtin::DebugSubscription;

#[derive(AzBuf, Clone, Debug, PartialEq, ServerboundGamePacket)]
pub struct ServerboundDebugSubscriptionRequest {
    pub subscriptions: Vec<DebugSubscription>,
}
