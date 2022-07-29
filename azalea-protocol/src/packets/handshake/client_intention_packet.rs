use crate::packets::ConnectionProtocol;
use azalea_buf::McBuf;
use packet_macros::ClientboundHandshakePacket;
use std::hash::Hash;

#[derive(Hash, Clone, Debug, McBuf, ClientboundHandshakePacket)]
pub struct ClientIntentionPacket {
    #[var]
    pub protocol_version: u32,
    pub hostname: String,
    pub port: u16,
    pub intention: ConnectionProtocol,
}
