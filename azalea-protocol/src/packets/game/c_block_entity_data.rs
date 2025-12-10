use azalea_buf::AzBuf;
use azalea_core::position::BlockPos;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::builtin::BlockEntityKind;
use simdnbt::owned::Nbt;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundBlockEntityData {
    pub pos: BlockPos,
    pub block_entity_type: BlockEntityKind,
    pub tag: Nbt,
}
