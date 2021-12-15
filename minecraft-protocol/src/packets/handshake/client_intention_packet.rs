use std::hash::Hash;

use async_trait::async_trait;
use tokio::io::BufReader;

use crate::{
    mc_buf,
    packets::{ConnectionProtocol, Packet},
};

#[derive(Hash, Clone, Debug)]
pub struct ClientIntentionPacket {
    pub protocol_version: u32,
    pub hostname: String,
    pub port: u16,
    /// 1 for status, 2 for login
    pub intention: ConnectionProtocol,
}

#[async_trait]
impl ClientIntentionPacket {
    fn get(self) -> Packet {
        Packet::ClientIntentionPacket(self)
    }

    fn write(&self, buf: &mut Vec<u8>) {
        mc_buf::write_varint(buf, self.protocol_version as i32);
        mc_buf::write_utf(buf, &self.hostname);
        mc_buf::write_short(buf, self.port);
        mc_buf::write_varint(buf, self.intention.clone() as i32);
    }

    async fn read<T: tokio::io::AsyncRead + std::marker::Unpin + std::marker::Send>(
        _buf: &mut BufReader<T>,
    ) -> Result<Packet, String> {
        Err("ClientIntentionPacket::parse not implemented".to_string())
        // Ok(ClientIntentionPacket {}.get())
    }
}
