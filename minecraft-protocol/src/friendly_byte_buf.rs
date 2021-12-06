//! Minecraft calls it a "friendly byte buffer".

use byteorder::{BigEndian, WriteBytesExt};

// const MAX_VARINT_SIZE: u32 = 5;
// const MAX_VARLONG_SIZE: u32 = 10;
// const DEFAULT_NBT_QUOTA: u32 = 2097152;
const MAX_STRING_LENGTH: u16 = 32767;
// const MAX_COMPONENT_STRING_LENGTH: u32 = 262144;

pub struct FriendlyByteBuf<'a> {
    source: &'a mut Vec<u8>,
}

impl<'a> FriendlyByteBuf<'a> {
    pub fn new(source: &'a mut Vec<u8>) -> FriendlyByteBuf<'a> {
        FriendlyByteBuf { source }
    }

    pub fn write_byte(&mut self, n: u8) {
        self.source.write_u8(n).unwrap();
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.source.extend_from_slice(bytes);
    }

    pub fn write_varint(&mut self, mut n: u32) {
        loop {
            if (n & 0xFFFFFF80) == 0 {
                self.write_byte(n as u8);
                return ();
            }
            self.write_byte((n & 0x7F | 0x80) as u8);
            n >>= 7;
        }
    }

    pub fn write_utf_with_len(&mut self, string: &String, len: usize) {
        if string.len() > len {
            panic!(
                "String too big (was {} bytes encoded, max {})",
                string.len(),
                len
            );
        }
        self.write_varint(string.len() as u32);
        self.write_bytes(string.as_bytes());
    }

    pub fn write_utf(&mut self, string: &String) {
        self.write_utf_with_len(string, MAX_STRING_LENGTH as usize);
    }

    pub fn write_short(&mut self, n: u16) {
        self.source.write_u16::<BigEndian>(n).unwrap();
    }
}
