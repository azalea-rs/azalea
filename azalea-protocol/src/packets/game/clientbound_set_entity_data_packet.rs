use azalea_buf::McBuf;
use azalea_entity::EntityMetadata;
use packet_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSetEntityDataPacket {
    #[var]
    pub id: u32,
    pub metadata: EntityMetadata,
}
