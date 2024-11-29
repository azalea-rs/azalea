use azalea_buf::AzBuf;
use azalea_chat::FormattedText;
use azalea_protocol_macros::ClientboundGamePacket;
use uuid::Uuid;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundResourcePackPush {
    pub id: Uuid,
    pub url: String,
    pub hash: String,
    pub required: bool,
    pub prompt: Option<FormattedText>,
}
