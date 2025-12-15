use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundLoginPacket;

#[derive(AzBuf, Clone, Debug, PartialEq, ServerboundLoginPacket)]
pub struct ServerboundKey {
    pub key_bytes: Vec<u8>,
    pub encrypted_challenge: Vec<u8>,
}
