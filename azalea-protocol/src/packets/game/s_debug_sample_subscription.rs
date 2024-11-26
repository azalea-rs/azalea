use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundDebugSampleSubscription {
    pub sample_type: RemoteDebugSampleType,
}

#[derive(Clone, Copy, Debug, McBuf)]
pub enum RemoteDebugSampleType {
    TickTime,
}
