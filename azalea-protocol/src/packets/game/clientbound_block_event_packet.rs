use azalea_block::BlockState;
use azalea_buf::McBuf;
use azalea_core::BlockPos;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundBlockEventPacket {
    pub pos: BlockPos,
    pub b0: u8,
    pub b1: u8,
    pub block: BlockState,
}
