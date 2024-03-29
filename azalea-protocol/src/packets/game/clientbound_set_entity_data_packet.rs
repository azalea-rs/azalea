use azalea_buf::McBuf;
use azalea_entity::EntityMetadataItems;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSetEntityDataPacket {
    #[var]
    pub id: u32,
    pub packed_items: EntityMetadataItems,
}
