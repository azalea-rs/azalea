#![no_main]

use std::io::Cursor;

use azalea_buf::AzBuf;
use azalea_protocol::packets::game::ClientboundLevelChunkWithLight;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = ClientboundLevelChunkWithLight::azalea_read(Default::default(), &mut Cursor::new(data));
});
