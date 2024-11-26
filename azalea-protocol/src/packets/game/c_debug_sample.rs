use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

use super::s_debug_sample_subscription::RemoteDebugSampleType;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundDebugSample {
    pub sample: Vec<u64>,
    pub debug_sample_type: RemoteDebugSampleType,
}
