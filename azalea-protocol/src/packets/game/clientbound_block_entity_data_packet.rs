use azalea_buf::McBuf;
use azalea_core::BlockPos;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundBlockEntityDataPacket {
    pub pos: BlockPos,
    // TODO: in vanilla this uses the block entity registry, we should have an enum in azalea-entity for this
    #[var]
    pub block_entity_type: u32,
    pub tag: azalea_nbt::Tag,
}
