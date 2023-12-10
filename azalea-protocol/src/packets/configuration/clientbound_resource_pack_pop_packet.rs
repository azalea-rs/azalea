use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundConfigurationPacket;
use uuid::Uuid;

#[derive(Clone, Debug, McBuf, ClientboundConfigurationPacket)]
pub struct ClientboundResourcePackPopPacket {
    pub id: Option<Uuid>,
}
