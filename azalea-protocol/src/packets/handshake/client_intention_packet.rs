use crate::{mc_buf::Writable, packets::ConnectionProtocol};
use packet_macros::HandshakePacket;
use std::hash::Hash;

use super::HandshakePacket;

#[derive(Hash, Clone, Debug, HandshakePacket)]
pub struct ClientIntentionPacket {
    #[varint]
    pub protocol_version: u32,
    pub hostname: String,
    pub port: u16,
    pub intention: ConnectionProtocol,
}

// impl ClientIntentionPacket {
//     pub fn get(self) -> HandshakePacket {
//         HandshakePacket::ClientIntentionPacket(self)
//     }

//     pub fn write(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
//         buf.write_varint(self.protocol_version as i32)?;
//         buf.write_utf(&self.hostname)?;
//         buf.write_short(self.port as i16)?;
//         buf.write_varint(self.intention as i32)?;
//         Ok(())
//     }

//     pub async fn read<T: tokio::io::AsyncRead + std::marker::Unpin + std::marker::Send>(
//         buf: &mut T,
//     ) -> Result<HandshakePacket, String> {
//         todo!()
//     }
// }
