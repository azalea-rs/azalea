use azalea_buf::McBuf;
use azalea_core::position::BlockPos;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::Block;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundBlockEventPacket {
    pub pos: BlockPos,
    pub action_id: u8,
    pub action_parameter: u8,
    pub block: Block,
}
