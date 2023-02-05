use azalea_buf::McBuf;
use azalea_chat::FormattedText;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundServerDataPacket {
    pub motd: Option<FormattedText>,
    pub icon_base64: Option<String>,
    pub previews_chat: bool,
    pub enforces_secure_chat: bool,
}
