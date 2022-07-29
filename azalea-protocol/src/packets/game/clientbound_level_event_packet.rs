use azalea_buf::McBuf;
use azalea_core::BlockPos;
use packet_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundLevelEventPacket {
    pub type_: i32,
    pub pos: BlockPos,
    pub data: i32,
    pub global_event: bool,
}
