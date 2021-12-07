//! Utilities for reading and writing for the Minecraft protocol

use std::io::{Cursor, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use tokio::io::{AsyncRead, AsyncReadExt, BufReader};

// const MAX_VARINT_SIZE: u32 = 5;
// const MAX_VARLONG_SIZE: u32 = 10;
// const DEFAULT_NBT_QUOTA: u32 = 2097152;
const MAX_STRING_LENGTH: u16 = 32767;
// const MAX_COMPONENT_STRING_LENGTH: u32 = 262144;

/// Read a single byte from the reader
pub async fn read_byte<T: AsyncRead + std::marker::Unpin>(
    buf: &mut BufReader<T>,
) -> Result<u8, String> {
    match AsyncReadExt::read_u8(buf).await {
        Ok(r) => Ok(r),
        Err(_) => Err("Error reading byte".to_string()),
    }
}

pub fn write_byte(buf: &mut Vec<u8>, n: u8) {
    WriteBytesExt::write_u8(buf, n).unwrap();
}

pub fn write_bytes(buf: &mut Vec<u8>, bytes: &[u8]) {
    buf.extend_from_slice(bytes);
}

// fast varints stolen from https://github.com/luojia65/mc-varint/blob/master/src/lib.rs#L67
/// Read a single varint from the reader and return the value, along with the number of bytes read
pub async fn read_varint<T: AsyncRead + std::marker::Unpin>(
    buf: &mut BufReader<T>,
) -> Result<(u32, u8), String> {
    let mut buffer = [0];
    let mut ans = 0;
    for i in 0..4 {
        buf.read_exact(&mut buffer)
            .await
            .or_else(|_| Err("Invalid VarInt".to_string()))?;
        ans |= ((buffer[0] & 0b0111_1111) as u32) << 7 * i;
        if buffer[0] & 0b1000_0000 == 0 {
            return Ok((ans, i + 1));
        }
    }
    Ok((ans, 5))
}

pub fn write_varint(buf: &mut Vec<u8>, mut value: u32) {
    let mut buffer = [0];
    while value != 0 {
        buffer[0] = (value & 0b0111_1111) as u8;
        value = (value >> 7) & (u32::max_value() >> 6);
        if value != 0 {
            buffer[0] |= 0b1000_0000;
        }
        buf.write(&buffer).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_write_varint() {
        let mut buf = Vec::new();
        write_varint(&mut buf, 123456);
        assert_eq!(buf, vec![192, 196, 7]);
    }

    #[tokio::test]
    async fn test_read_varint() {
        let mut buf = BufReader::new(Cursor::new(vec![192, 196, 7]));
        assert_eq!(read_varint(&mut buf).await.unwrap(), (123456, 3));
    }

    #[tokio::test]
    async fn test_read_varint_longer() {
        let mut buf = BufReader::new(Cursor::new(vec![138, 56, 0, 135, 56, 123]));
        assert_eq!(read_varint(&mut buf).await.unwrap(), (7178, 2));
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
    WriteBytesExt::write_u16::<BigEndian>(buf, n).unwrap();
}
