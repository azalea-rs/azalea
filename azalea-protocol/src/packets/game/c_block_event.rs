use azalea_buf::AzBuf;
use azalea_core::position::BlockPos;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::builtin::BlockKind;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundBlockEvent {
    pub pos: BlockPos,
    pub action_id: u8,
    pub action_parameter: u8,
    pub block: BlockKind,
}
