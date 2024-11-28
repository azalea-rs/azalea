use azalea_buf::AzBuf;
use azalea_entity::EntityMetadataItems;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundSetEntityData {
    #[var]
    pub id: u32,
    pub packed_items: EntityMetadataItems,
}
