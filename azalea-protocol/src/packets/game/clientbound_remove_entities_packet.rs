use packet_macros::GamePacket;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundRemoveEntitiesPacket {
    #[var]
    pub entity_ids: Vec<u32>,
}
