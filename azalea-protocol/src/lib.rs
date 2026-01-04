#![doc = include_str!("../README.md")]
// this is necessary for thiserror backtraces
#![feature(error_generic_member_access)]

pub mod address;
pub mod common;
#[cfg(feature = "connecting")]
pub mod connect;
pub mod packets;
pub mod read;
pub mod resolve;
pub mod write;

// re-export to make it easier for users to have the correct version
pub use simdnbt;

#[doc(hidden)]
#[deprecated(note = "renamed to `resolve`.")]
pub mod resolver {
    pub use super::resolve::*;
}

#[doc(hidden)]
#[deprecated(note = "moved to `address::ServerAddr`.")]
pub type ServerAddress = address::ServerAddr;

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use uuid::Uuid;

    use crate::{
        packets::{
            Packet,
            game::s_chat::{LastSeenMessagesUpdate, ServerboundChat},
            login::{ServerboundLoginPacket, s_hello::ServerboundHello},
        },
        read::{compression_decoder, read_packet},
        write::{compression_encoder, serialize_packet, write_packet},
    };

    #[tokio::test]
    async fn test_hello_packet() {
        let packet = ServerboundHello {
            name: "test".to_owned(),
            profile_id: Uuid::nil(),
        };
        let mut stream = Vec::new();
        write_packet(&packet.into_variant(), &mut stream, None, &mut None)
            .await
            .unwrap();

        assert_eq!(
            stream,
            [
                22, 0, 4, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
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
            name: "test".to_owned(),
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

        println!("{buf:?}");

        compression_decoder(&mut Cursor::new(&buf), compression_threshold).unwrap();
    }
}
