use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundRemoveEntitiesPacket {
    #[var]
    pub entity_ids: Vec<u32>,
}
