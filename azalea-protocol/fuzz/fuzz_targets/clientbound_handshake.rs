#![no_main]

use std::io::Cursor;

use azalea_protocol::{packets::handshake::ClientboundHandshakePacket, read::deserialize_packet};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = deserialize_packet::<ClientboundHandshakePacket>(&mut Cursor::new(data));
});
