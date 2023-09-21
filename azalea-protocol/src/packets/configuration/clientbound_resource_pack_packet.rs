use azalea_buf::McBuf;
use azalea_chat::FormattedText;
use azalea_protocol_macros::ClientboundConfigurationPacket;

#[derive(Clone, Debug, McBuf, ClientboundConfigurationPacket)]
pub struct ClientboundResourcePackPacket {
    pub url: String,
    pub hash: String,
    pub required: bool,
    pub prompt: Option<FormattedText>,
}
