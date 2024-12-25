//! A low-level crate to send and receive Minecraft packets.
//!
//! You should probably use [`azalea`] or [`azalea_client`] instead, as
//! `azalea_protocol` delegates much of the work, such as auth, to the user of
//! the crate.
//!
//! [`azalea`]: https://crates.io/crates/azalea
//! [`azalea_client`]: https://crates.io/crates/azalea-client
//!
//! See [`crate::connect::Connection`] for an example.

// this is necessary for thiserror backtraces
#![feature(error_generic_member_access)]

use std::{fmt::Display, net::SocketAddr, str::FromStr};

pub mod common;
#[cfg(feature = "connecting")]
pub mod connect;
#[cfg(feature = "packets")]
pub mod packets;
pub mod read;
pub mod resolver;
pub mod write;

/// A host and port. It's possible that the port doesn't resolve to anything.
///
/// # Examples
///
/// `ServerAddress` implements TryFrom<&str>, so you can use it like this:
/// ```
/// use azalea_protocol::ServerAddress;
///
/// let addr = ServerAddress::try_from("localhost:25565").unwrap();
/// assert_eq!(addr.host, "localhost");
/// assert_eq!(addr.port, 25565);
/// ```
#[derive(Debug, Clone)]
pub struct ServerAddress {
    pub host: String,
    pub port: u16,
}

impl TryFrom<&str> for ServerAddress {
    type Error = String;

    /// Convert a Minecraft server address (host:port, the port is optional) to
    /// a `ServerAddress`
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
impl TryFrom<String> for ServerAddress {
    type Error = String;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        ServerAddress::try_from(string.as_str())
    }
}

impl From<SocketAddr> for ServerAddress {
    /// Convert an existing `SocketAddr` into a `ServerAddress`. This just
    /// converts the ip to a string and passes along the port. The resolver
    /// will realize it's already an IP address and not do any DNS requests.
    fn from(addr: SocketAddr) -> Self {
        ServerAddress {
            host: addr.ip().to_string(),
            port: addr.port(),
        }
    }
}

impl Display for ServerAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

/// Serde deserialization for ServerAddress. This is useful for config file
/// usage.
impl<'de> serde::Deserialize<'de> for ServerAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        ServerAddress::try_from(string.as_str()).map_err(serde::de::Error::custom)
    }
}

/// Serde serialization for ServerAddress. This uses the Display impl, so it
/// will serialize to a string.
impl serde::Serialize for ServerAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use uuid::Uuid;

    use crate::{
        packets::{
            game::s_chat::{LastSeenMessagesUpdate, ServerboundChat},
            login::{s_hello::ServerboundHello, ServerboundLoginPacket},
            Packet,
        },
        read::{compression_decoder, read_packet},
        write::{compression_encoder, serialize_packet, write_packet},
    };

    #[tokio::test]
    async fn test_hello_packet() {
        let packet = ServerboundHello {
            name: "test".to_string(),
            profile_id: Uuid::nil(),
        };
        let mut stream = Vec::new();
        write_packet(&packet.into_variant(), &mut stream, None, &mut None)
            .await
            .unwrap();

        assert_eq!(
            stream,
            [22, 0, 4, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );

        let mut stream = Cursor::new(stream);

        let _ = read_packet::<ServerboundLoginPacket, _>(
            &mut stream,
            &mut Cursor::new(Vec::new()),
            None,
            &mut None,
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn test_double_hello_packet() {
        let packet = ServerboundHello {
            name: "test".to_string(),
            profile_id: Uuid::nil(),
        }
        .into_variant();
        let mut stream = Vec::new();
        write_packet(&packet, &mut stream, None, &mut None)
            .await
            .unwrap();
        write_packet(&packet, &mut stream, None, &mut None)
            .await
            .unwrap();
        let mut stream = Cursor::new(stream);

        let mut buffer = Cursor::new(Vec::new());

        let _ = read_packet::<ServerboundLoginPacket, _>(&mut stream, &mut buffer, None, &mut None)
            .await
            .unwrap();
        let _ = read_packet::<ServerboundLoginPacket, _>(&mut stream, &mut buffer, None, &mut None)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_read_long_compressed_chat() {
        let compression_threshold = 256;

        let buf = serialize_packet(
            &ServerboundChat {
                message: "a".repeat(256),
                timestamp: 0,
                salt: 0,
                signature: None,
                last_seen_messages: LastSeenMessagesUpdate::default(),
            }
            .into_variant(),
        )
        .unwrap();

        let buf = compression_encoder(&buf, compression_threshold).unwrap();

        println!("{:?}", buf);

        compression_decoder(&mut Cursor::new(&buf), compression_threshold).unwrap();
    }
}
