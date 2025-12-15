use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundDebugSample {
    pub sample: Vec<u64>,
    pub debug_sample_type: RemoteDebugSampleType,
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq)]
pub enum RemoteDebugSampleType {
    TickTime,
}
