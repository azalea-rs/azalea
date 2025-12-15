use azalea_buf::AzBuf;
use azalea_core::position::BlockPos;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::builtin::BlockEntityKind;
use simdnbt::owned::Nbt;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundBlockEntityData {
    pub pos: BlockPos,
    pub block_entity_type: BlockEntityKind,
    pub tag: Nbt,
}
