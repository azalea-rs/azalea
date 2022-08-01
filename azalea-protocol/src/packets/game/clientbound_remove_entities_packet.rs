use azalea_buf::McBuf;
use packet_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundRemoveEntitiesPacket {
    #[var]
    pub entity_ids: Vec<u32>,
}
