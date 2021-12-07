//! Utilities for reading and writing for the Minecraft protocol

use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use tokio::io::AsyncReadExt;

// const MAX_VARINT_SIZE: u32 = 5;
// const MAX_VARLONG_SIZE: u32 = 10;
// const DEFAULT_NBT_QUOTA: u32 = 2097152;
const MAX_STRING_LENGTH: u16 = 32767;
// const MAX_COMPONENT_STRING_LENGTH: u32 = 262144;

pub async fn read_byte(buf: &mut Cursor<Vec<u8>>) -> Result<u8, String> {
    match AsyncReadExt::read_u8(buf).await {
        Ok(r) => Ok(r),
        Err(_) => Err("Error reading byte".to_string()),
    }
}

pub fn write_byte(buf: &mut Vec<u8>, n: u8) {
    buf.write_u8(n).unwrap();
}

pub fn write_bytes(buf: &mut Vec<u8>, bytes: &[u8]) {
    buf.extend_from_slice(bytes);
}

pub async fn read_varint(buf: &mut Cursor<Vec<u8>>) -> Result<u32, String> {
    let mut value: u32 = 0;
    let mut length: u32 = 0;
    let mut current_byte: u8;

    loop {
        current_byte = read_byte(buf).await?;
        value |= ((current_byte & 0x7F) as u32) << (length * 7);

        length += 1;
        if length > 5 {
            return Err("VarInt too big".to_string());
        }

        if (value & 0x80) != 0x80 {
            return Ok(value);
        }
    }
}

pub fn write_varint(buf: &mut Vec<u8>, mut n: u32) {
    loop {
        if (n & 0xFFFFFF80) == 0 {
            write_byte(buf, n as u8);
            return ();
        }
        write_byte(buf, (n & 0x7F | 0x80) as u8);
        n >>= 7;
    }
}

pub fn write_utf_with_len(buf: &mut Vec<u8>, string: &String, len: usize) {
    if string.len() > len {
        panic!(
            "String too big (was {} bytes encoded, max {})",
            string.len(),
            len
        );
    }
    write_varint(buf, string.len() as u32);
    write_bytes(buf, string.as_bytes());
}

pub fn write_utf(buf: &mut Vec<u8>, string: &String) {
    write_utf_with_len(buf, string, MAX_STRING_LENGTH as usize);
}

pub fn write_short(buf: &mut Vec<u8>, n: u16) {
    buf.write_u16::<BigEndian>(n).unwrap();
}
