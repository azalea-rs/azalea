use packet_macros::{GamePacket, McBuf};

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundRemoveEntitiesPacket {
    #[var]
    pub entity_ids: Vec<u32>,
}
