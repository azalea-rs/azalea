use packet_macros::{LoginPacket, McBuf};
use std::hash::Hash;

#[derive(Hash, Clone, Debug, McBuf, LoginPacket)]
pub struct ServerboundKeyPacket {
    pub shared_secret: Vec<u8>,
    pub nonce: Vec<u8>,
}
