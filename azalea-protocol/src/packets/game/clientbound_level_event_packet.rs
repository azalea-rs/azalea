use azalea_buf::McBuf;
use azalea_core::BlockPos;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundLevelEventPacket {
    pub event_type: u32,
    pub pos: BlockPos,
    pub data: u32,
    pub global_event: bool,
}
