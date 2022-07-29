use azalea_buf::McBuf;
use packet_macros::ServerboundLoginPacket;
use uuid::Uuid;

#[derive(Clone, Debug, McBuf, ServerboundLoginPacket)]
pub struct ServerboundHelloPacket {
    pub username: String,
    pub public_key: Option<ProfilePublicKeyData>,
    pub profile_id: Option<Uuid>,
}

#[derive(Clone, Debug, McBuf)]
pub struct ProfilePublicKeyData {
    pub expires_at: u64,
    pub key: Vec<u8>,
    pub key_signature: Vec<u8>,
}
