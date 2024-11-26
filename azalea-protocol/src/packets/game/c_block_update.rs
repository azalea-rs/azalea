use azalea_block::BlockState;
use azalea_buf::McBuf;
use azalea_core::position::BlockPos;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundBlockUpdate {
    pub pos: BlockPos,
    pub block_state: BlockState,
}
