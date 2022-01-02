use crate::{
    mc_buf::{Readable, Writable},
    packets::ConnectionProtocol,
};
use num_traits::FromPrimitive;
use packet_macros::HandshakePacket;
use std::hash::Hash;

#[derive(Hash, Clone, Debug, HandshakePacket)]
pub struct ClientIntentionPacket {
    #[varint]
    pub protocol_version: u32,
    pub hostname: String,
    pub port: u16,
    pub intention: ConnectionProtocol,
}
