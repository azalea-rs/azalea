use azalea_buf::AzBuf;
use azalea_core::entity_id::MinecraftEntityId;
use azalea_entity::EntityMetadataItems;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundSetEntityData {
    #[var]
    pub id: MinecraftEntityId,
    pub packed_items: EntityMetadataItems,
}
