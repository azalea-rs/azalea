use azalea_buf::McBuf;
use azalea_chat::FormattedText;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket, PartialEq)]
pub struct ClientboundSystemChatPacket {
    pub content: FormattedText,
    pub overlay: bool,
}
