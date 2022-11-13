use azalea_buf::McBuf;
use azalea_crypto::MessageSignature;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundDeleteChatPacket {
    pub message_signature: MessageSignature,
}
