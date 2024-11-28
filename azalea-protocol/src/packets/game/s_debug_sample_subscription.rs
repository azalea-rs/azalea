use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, AzBuf, ServerboundGamePacket)]
pub struct ServerboundDebugSampleSubscription {
    pub sample_type: RemoteDebugSampleType,
}

#[derive(Clone, Copy, Debug, AzBuf)]
pub enum RemoteDebugSampleType {
    TickTime,
}
