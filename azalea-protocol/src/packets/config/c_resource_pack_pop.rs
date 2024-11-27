use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundConfigPacket;
use uuid::Uuid;

#[derive(Clone, Debug, AzBuf, ClientboundConfigPacket)]
pub struct ClientboundResourcePackPop {
    pub id: Option<Uuid>,
}
