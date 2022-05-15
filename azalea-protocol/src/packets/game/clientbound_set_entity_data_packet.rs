use crate::mc_buf::EntityMetadata;
use packet_macros::{GamePacket, McBuf};

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundSetEntityDataPacket {
    #[var]
    pub id: i32,
    pub metadata: EntityMetadata,
}
