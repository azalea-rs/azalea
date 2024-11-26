use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use uuid::Uuid;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundPlayerInfoRemove {
    pub profile_ids: Vec<Uuid>,
}
