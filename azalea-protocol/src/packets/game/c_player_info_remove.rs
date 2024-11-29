use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use uuid::Uuid;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundPlayerInfoRemove {
    pub profile_ids: Vec<Uuid>,
}
