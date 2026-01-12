#![no_main]

use std::io::Cursor;

use azalea_protocol::{packets::config::ServerboundConfigPacket, read::deserialize_packet};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = deserialize_packet::<ServerboundConfigPacket>(&mut Cursor::new(data));
});
