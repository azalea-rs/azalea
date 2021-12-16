use std::hash::Hash;

use tokio::io::BufReader;

use crate::{mc_buf::Writable, packets::ConnectionProtocol};

use super::HandshakePacket;

#[derive(Hash, Clone, Debug)]
pub struct ClientIntentionPacket {
    pub protocol_version: u32,
    pub hostname: String,
    pub port: u16,
    /// 1 for status, 2 for login
    pub intention: ConnectionProtocol,
}

impl ClientIntentionPacket {
    pub fn get(self) -> HandshakePacket {
        HandshakePacket::ClientIntentionPacket(self)
    }

    pub fn write(&self, buf: &mut Vec<u8>) {
        buf.write_varint(self.protocol_version as i32).unwrap();
        buf.write_utf(&self.hostname).unwrap();
        buf.write_short(self.port).unwrap();
        buf.write_varint(self.intention.clone() as i32).unwrap();
    }

    pub async fn read<T: tokio::io::AsyncRead + std::marker::Unpin + std::marker::Send>(
        _buf: &mut BufReader<T>,
    ) -> Result<HandshakePacket, String> {
        Err("ClientIntentionPacket::parse not implemented".to_string())
        // Ok(ClientIntentionPacket {}.get())
    }
}
