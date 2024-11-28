use azalea_buf::AzBuf;
use azalea_chat::FormattedText;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundServerData {
    pub motd: FormattedText,
    pub icon_bytes: Option<Vec<u8>>,
}
