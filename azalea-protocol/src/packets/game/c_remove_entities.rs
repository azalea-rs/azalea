use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundRemoveEntities {
    #[var]
    pub entity_ids: Vec<u32>,
}
