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

// impl ClientIntentionPacket {
//     pub fn get(self) -> HandshakePacket {
//         HandshakePacket::ClientIntentionPacket(self)
//     }

//     pub fn write(&self, buf: &mut Vec<u8>) {
//         buf.write_varint(self.protocol_version as i32).unwrap();
//         buf.write_utf(&self.hostname).unwrap();
//         buf.write_short(self.port).unwrap();
//         buf.write_varint(self.intention.clone() as i32).unwrap();
//     }

//     pub async fn read<T: tokio::io::AsyncRead + std::marker::Unpin + std::marker::Send>(
//         buf: &mut T,
//     ) -> Result<HandshakePacket, String> {
//         let protocol_version = buf.read_varint().await? as u32;
//         let hostname = buf.read_utf().await?;
//         let port = buf.read_short().await? as u16;
//         let intention = buf.read_varint().await?;

//         Ok(HandshakePacket::ClientIntentionPacket(
//             ClientIntentionPacket {
//                 protocol_version,
//                 hostname,
//                 port,
//                 intention: ConnectionProtocol::from_i32(intention)
//                     .ok_or_else(|| "Invalid intention".to_string())?,
//             },
//         ))
//     }
// }
