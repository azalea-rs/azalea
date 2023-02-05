use azalea_buf::McBuf;
use azalea_chat::FormattedText;
use azalea_protocol_macros::ClientboundLoginPacket;

#[derive(Clone, Debug, McBuf, ClientboundLoginPacket)]
pub struct ClientboundLoginDisconnectPacket {
    pub reason: FormattedText,
}
