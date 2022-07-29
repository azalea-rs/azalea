use azalea_buf::McBuf;
use azalea_crypto::MessageSignature;
use packet_macros::GamePacket;

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundDeleteChatPacket {
    pub message_signature: MessageSignature,
}
