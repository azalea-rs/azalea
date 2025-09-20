use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;
use azalea_registry::DebugSubscription;

#[derive(Clone, Debug, AzBuf, ServerboundGamePacket)]
pub struct ServerboundDebugSubscriptionRequest {
    pub subscriptions: Vec<DebugSubscription>,
}
