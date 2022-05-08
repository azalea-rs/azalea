use crate::mc_buf::EntityMetadata;
use packet_macros::GamePacket;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundSetEntityDataPacket {
    #[var]
    pub id: i32,
    pub metadata: EntityMetadata,
}
