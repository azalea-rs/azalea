use azalea_buf::McBuf;
use packet_macros::GamePacket;

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundRemoveEntitiesPacket {
    #[var]
    pub entity_ids: Vec<u32>,
}
