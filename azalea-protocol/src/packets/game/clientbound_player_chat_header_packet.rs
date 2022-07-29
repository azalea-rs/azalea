use azalea_buf::McBuf;
use azalea_crypto::{MessageSignature, SignedMessageHeader};
use packet_macros::GamePacket;

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundPlayerChatHeaderPacket {
    pub header: SignedMessageHeader,
    pub header_signature: MessageSignature,
    pub body_digest: Vec<u8>,
}
