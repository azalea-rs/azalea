use super::LoginPacket;
use crate::mc_buf::Writable;
use packet_macros::LoginPacket;
use std::hash::Hash;

#[derive(Hash, Clone, Debug, LoginPacket)]
pub struct ServerboundKeyPacket {
    pub shared_secret: Vec<u8>,
    pub nonce: Vec<u8>,
}
