//! This lib is responsible for parsing Minecraft packets.

use std::net::IpAddr;
use std::str::FromStr;

#[cfg(feature = "connecting")]
pub mod connect;
#[cfg(feature = "packets")]
pub mod packets;
pub mod read;
pub mod resolver;
pub mod write;

#[derive(Debug)]
pub struct ServerAddress {
    pub host: String,
    pub port: u16,
}

#[derive(Debug)]
pub struct ServerIpAddress {
    pub ip: IpAddr,
    pub port: u16,
}

// impl try_from for ServerAddress
impl<'a> TryFrom<&'a str> for ServerAddress {
    type Error = String;

    /// Convert a Minecraft server address (host:port, the port is optional) to a ServerAddress
    fn try_from(string: &str) -> Result<Self, Self::Error> {
        if string.is_empty() {
            return Err("Empty string".to_string());
        }
        let mut parts = string.split(':');
        let host = parts.next().ok_or("No host specified")?.to_string();
        // default the port to 25565
        let port = parts.next().unwrap_or("25565");
        let port = u16::from_str(port).map_err(|_| "Invalid port specified")?;
        Ok(ServerAddress { host, port })
    }
}

#[cfg(feature = "connecting")]
pub async fn connect(address: ServerAddress) -> Result<(), Box<dyn std::error::Error>> {
    use log::debug;

    let resolved_address = resolver::resolve_address(&address).await;
    debug!("Resolved address: {:?}", resolved_address);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{
        packets::login::{
            serverbound_hello_packet::{ProfilePublicKeyData, ServerboundHelloPacket},
            ServerboundLoginPacket,
        },
        read::read_packet,
        write::write_packet,
    };
    use bytes::BytesMut;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_hello_packet() {
        let packet = ServerboundHelloPacket {
            username: "test".to_string(),
            public_key: Some(ProfilePublicKeyData {
                expires_at: 0,
                key: b"idontthinkthisreallymattersijustwantittobelongforthetest".to_vec(),
                key_signature: b"idontthinkthisreallymattersijustwantittobelongforthetest".to_vec(),
            }),
            profile_id: Some(Uuid::from_u128(0)),
        }
        .get();
        let mut stream = Vec::new();
        write_packet(packet, &mut stream, None, &mut None)
            .await
            .unwrap();

        println!("stream: {stream:?}");

        let mut stream = Cursor::new(stream);

        let _ = read_packet::<ServerboundLoginPacket, _>(
            &mut stream,
            &mut BytesMut::new(),
            None,
            &mut None,
        )
        .await
        .unwrap();
    }
}
