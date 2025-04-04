use azalea_buf::AzBuf;
use azalea_crypto::{MessageSignature, SignedMessageHeader};
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundPlayerChatHeader {
    pub header: SignedMessageHeader,
    pub header_signature: MessageSignature,
    pub body_digest: Vec<u8>,
}
