use super::LoginPacket;
use crate::mc_buf::{ByteArray, Writable};
use packet_macros::LoginPacket;
use std::hash::Hash;

#[derive(Hash, Clone, Debug, LoginPacket)]
pub struct ServerboundKeyPacket {
    pub shared_secret: ByteArray,
    pub nonce: ByteArray,
}
