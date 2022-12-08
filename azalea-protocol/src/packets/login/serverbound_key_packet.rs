use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundLoginPacket;

#[derive(Clone, Debug, McBuf, ServerboundLoginPacket)]
pub struct ServerboundKeyPacket {
    pub key_bytes: Vec<u8>,
    pub encrypted_challenge: Vec<u8>,
}
