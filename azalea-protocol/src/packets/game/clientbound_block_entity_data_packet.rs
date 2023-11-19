use azalea_buf::McBuf;
use azalea_core::position::BlockPos;
use azalea_protocol_macros::ClientboundGamePacket;
use simdnbt::owned::NbtTag;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundBlockEntityDataPacket {
    pub pos: BlockPos,
    pub block_entity_type: azalea_registry::BlockEntityKind,
    pub tag: NbtTag,
}
