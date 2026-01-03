use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundConfigPacket;
use uuid::Uuid;

#[derive(AzBuf, ClientboundConfigPacket, Clone, Debug, PartialEq)]
pub struct ClientboundResourcePackPop {
    pub id: Option<Uuid>,
}
