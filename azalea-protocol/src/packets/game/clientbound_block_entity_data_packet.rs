use azalea_buf::McBuf;
use azalea_core::BlockPos;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundBlockEntityDataPacket {
    pub pos: BlockPos,
    pub block_entity_type: azalea_registry::BlockEntityType,
    pub tag: azalea_nbt::Tag,
}
