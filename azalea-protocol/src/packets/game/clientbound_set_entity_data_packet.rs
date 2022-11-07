use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_world::entity::EntityMetadataItems;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSetEntityDataPacket {
    #[var]
    pub id: u32,
    pub packed_items: EntityMetadataItems,
}
