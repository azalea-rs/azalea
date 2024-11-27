use azalea_buf::AzBuf;
use azalea_chat::FormattedText;
use azalea_protocol_macros::ClientboundLoginPacket;

#[derive(Clone, Debug, AzBuf, ClientboundLoginPacket)]
pub struct ClientboundCookieRequest {
    pub key: FormattedText,
}
