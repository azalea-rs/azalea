use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;
use uuid::Uuid;

#[derive(Clone, Debug, AzBuf, ServerboundGamePacket)]
pub struct ServerboundChatSessionUpdate {
    pub chat_session: RemoteChatSessionData,
}

#[derive(Clone, Debug, PartialEq, Eq, AzBuf)]
pub struct RemoteChatSessionData {
    pub session_id: Uuid,
    pub profile_public_key: ProfilePublicKeyData,
}

#[derive(Clone, Debug, AzBuf, PartialEq, Eq)]
pub struct ProfilePublicKeyData {
    pub expires_at: u64,
    pub key: Vec<u8>,
    pub key_signature: Vec<u8>,
}
