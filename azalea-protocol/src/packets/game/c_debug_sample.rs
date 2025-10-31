use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, PartialEq, Eq, ClientboundGamePacket)]
pub struct ClientboundDebugSample {
    pub sample: Vec<u64>,
    pub debug_sample_type: RemoteDebugSampleType,
}

#[derive(Clone, Copy, Debug, AzBuf, PartialEq, Eq)]
pub enum RemoteDebugSampleType {
    TickTime,
}
