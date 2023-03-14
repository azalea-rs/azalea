use azalea_buf::McBuf;
use azalea_chat::FormattedText;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundServerDataPacket {
    pub motd: FormattedText,
    pub icon_bytes: Option<Vec<u8>>,
    pub enforces_secure_chat: bool,
}
