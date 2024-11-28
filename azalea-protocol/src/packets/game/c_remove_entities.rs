use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundRemoveEntities {
    #[var]
    pub entity_ids: Vec<u32>,
}
