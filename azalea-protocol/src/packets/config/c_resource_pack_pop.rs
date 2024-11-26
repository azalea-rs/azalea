use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundConfigPacket;
use uuid::Uuid;

#[derive(Clone, Debug, McBuf, ClientboundConfigPacket)]
pub struct ClientboundResourcePackPop {
    pub id: Option<Uuid>,
}
