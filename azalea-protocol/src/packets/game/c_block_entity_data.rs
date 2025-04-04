use azalea_buf::AzBuf;
use azalea_core::position::BlockPos;
use azalea_protocol_macros::ClientboundGamePacket;
use simdnbt::owned::Nbt;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundBlockEntityData {
    pub pos: BlockPos,
    pub block_entity_type: azalea_registry::BlockEntityKind,
    pub tag: Nbt,
}
