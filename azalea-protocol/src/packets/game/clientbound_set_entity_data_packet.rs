use azalea_buf::McBuf;
use azalea_world::entity::EntityMetadata;
use packet_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSetEntityDataPacket {
    #[var]
    pub id: u32,
    pub packed_items: EntityMetadata,
}
