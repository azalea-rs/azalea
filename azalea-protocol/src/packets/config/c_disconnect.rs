use azalea_buf::AzBuf;
use azalea_chat::FormattedText;
use azalea_protocol_macros::ClientboundConfigPacket;

#[derive(Clone, Debug, AzBuf, ClientboundConfigPacket)]
pub struct ClientboundDisconnect {
    pub reason: FormattedText,
}
