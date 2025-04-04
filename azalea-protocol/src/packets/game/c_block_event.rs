use azalea_buf::AzBuf;
use azalea_core::position::BlockPos;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::Block;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundBlockEvent {
    pub pos: BlockPos,
    pub action_id: u8,
    pub action_parameter: u8,
    pub block: Block,
}
