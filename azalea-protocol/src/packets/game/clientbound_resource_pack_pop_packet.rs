use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use uuid::Uuid;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundResourcePackPopPacket {
    pub id: Option<Uuid>,
}
