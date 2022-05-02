use crate::packets::ConnectionProtocol;
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
