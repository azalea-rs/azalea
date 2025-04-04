use azalea_buf::AzBuf;
use azalea_core::position::BlockPos;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundLevelEvent {
    pub event_type: u32,
    pub pos: BlockPos,
    pub data: u32,
    pub global_event: bool,
}
