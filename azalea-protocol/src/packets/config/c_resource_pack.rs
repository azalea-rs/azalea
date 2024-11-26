use azalea_buf::McBuf;
use azalea_chat::FormattedText;
use azalea_protocol_macros::ClientboundConfigPacket;

#[derive(Clone, Debug, McBuf, ClientboundConfigPacket)]
pub struct ClientboundResourcePack {
    pub url: String,
    pub hash: String,
    pub required: bool,
    pub prompt: Option<FormattedText>,
}
