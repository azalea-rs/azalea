use azalea_buf::McBuf;
use azalea_world::entity::EntityMetadata;
use packet_macros::GamePacket;

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundSetEntityDataPacket {
    #[var]
    pub id: u32,
    pub metadata: EntityMetadata,
}
