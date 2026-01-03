use azalea_block::BlockState;
use azalea_buf::AzBuf;
use azalea_core::position::BlockPos;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundBlockUpdate {
    pub pos: BlockPos,
    pub block_state: BlockState,
}
