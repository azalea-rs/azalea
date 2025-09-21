use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, AzBuf, PartialEq, ServerboundGamePacket)]
pub struct ServerboundDebugSampleSubscription {
    pub sample_type: RemoteDebugSampleType,
}

#[derive(Clone, Copy, Debug, AzBuf, PartialEq)]
pub enum RemoteDebugSampleType {
    TickTime,
}
