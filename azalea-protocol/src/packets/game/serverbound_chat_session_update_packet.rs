use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundGamePacket;
use uuid::Uuid;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundChatSessionUpdatePacket {
    pub chat_session: RemoteChatSessionData,
}

#[derive(Clone, Debug, PartialEq, Eq, McBuf)]
pub struct RemoteChatSessionData {
    pub session_id: Uuid,
    pub profile_public_key: ProfilePublicKeyData,
}

#[derive(Clone, Debug, McBuf, PartialEq, Eq)]
pub struct ProfilePublicKeyData {
    pub expires_at: u64,
    pub key: Vec<u8>,
    pub key_signature: Vec<u8>,
}
