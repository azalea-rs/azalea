use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundLoginPacket;

#[derive(Clone, Debug, AzBuf, ServerboundLoginPacket)]
pub struct ServerboundKey {
    pub key_bytes: Vec<u8>,
    pub encrypted_challenge: Vec<u8>,
}
