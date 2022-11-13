use azalea_buf::McBuf;
use azalea_crypto::{MessageSignature, SignedMessageHeader};
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundPlayerChatHeaderPacket {
    pub header: SignedMessageHeader,
    pub header_signature: MessageSignature,
    pub body_digest: Vec<u8>,
}
