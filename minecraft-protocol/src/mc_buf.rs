//! Utilities for reading and writing for the Minecraft protocol

use std::io::Write;

use async_trait::async_trait;
use byteorder::{BigEndian, WriteBytesExt};
use tokio::io::{AsyncRead, AsyncReadExt};

// const DEFAULT_NBT_QUOTA: u32 = 2097152;
const MAX_STRING_LENGTH: u16 = 32767;
// const MAX_COMPONENT_STRING_LENGTH: u32 = 262144;

pub fn write_byte(buf: &mut Vec<u8>, n: u8) {
    WriteBytesExt::write_u8(buf, n).unwrap();
}

pub fn write_bytes(buf: &mut Vec<u8>, bytes: &[u8]) {
    buf.extend_from_slice(bytes);
}

pub fn write_varint(buf: &mut Vec<u8>, mut value: i32) {
    let mut buffer = [0];
    if value == 0 {
        buf.write_all(&buffer).unwrap();
    }
    while value != 0 {
        buffer[0] = (value & 0b0111_1111) as u8;
        value = (value >> 7) & (i32::max_value() >> 6);
        if value != 0 {
            buffer[0] |= 0b1000_0000;
        }
        buf.write_all(&buffer).unwrap();
    }
}

pub fn write_utf_with_len(buf: &mut Vec<u8>, string: &str, len: usize) {
    if string.len() > len {
        panic!(
            "String too big (was {} bytes encoded, max {})",
            string.len(),
            len
        );
    }
    write_varint(buf, string.len() as i32);
    write_bytes(buf, string.as_bytes());
}
pub fn write_utf(buf: &mut Vec<u8>, string: &str) {
    write_utf_with_len(buf, string, MAX_STRING_LENGTH.into());
}

pub fn write_short(buf: &mut Vec<u8>, n: u16) {
    WriteBytesExt::write_u16::<BigEndian>(buf, n).unwrap();
}

pub fn write_byte_array(buf: &mut Vec<u8>, bytes: &[u8]) {
    write_varint(buf, bytes.len() as i32);
    write_bytes(buf, bytes);
}

#[async_trait]
pub trait Readable {
    async fn read_varint(&mut self) -> Result<(i32, u8), String>;
    async fn read_byte_array(&mut self) -> Result<Vec<u8>, String>;
    async fn read_bytes(&mut self, n: usize) -> Result<Vec<u8>, String>;
    async fn read_utf(&mut self) -> Result<String, String>;
    async fn read_utf_with_len(&mut self, max_length: u32) -> Result<String, String>;
    async fn read_byte(&mut self) -> Result<u8, String>;
}

#[async_trait]
impl<R> Readable for R
where
    R: AsyncRead + std::marker::Unpin + std::marker::Send,
{
    // fast varints stolen from https://github.com/luojia65/mc-varint/blob/master/src/lib.rs#L67
    /// Read a single varint from the reader and return the value, along with the number of bytes read
    async fn read_varint(&mut self) -> Result<(i32, u8), String> {
        let mut buffer = [0];
        let mut ans = 0;
        for i in 0..4 {
            self.read_exact(&mut buffer)
                .await
                .map_err(|_| "Invalid VarInt".to_string())?;
            ans |= ((buffer[0] & 0b0111_1111) as i32) << (7 * i);
            if buffer[0] & 0b1000_0000 == 0 {
                return Ok((ans, i + 1));
            }
        }
        Ok((ans, 5))
    }

    async fn read_byte_array(&mut self) -> Result<Vec<u8>, String> {
        let length = self.read_varint().await?.0 as usize;
        Ok(self.read_bytes(length).await?)
    }

    async fn read_bytes(&mut self, n: usize) -> Result<Vec<u8>, String> {
        let mut bytes = vec![0; n];
        match AsyncReadExt::read_exact(self, &mut bytes).await {
            Ok(_) => Ok(bytes),
            Err(_) => Err("Error reading bytes".to_string()),
        }
    }

    async fn read_utf(&mut self) -> Result<String, String> {
        self.read_utf_with_len(MAX_STRING_LENGTH.into()).await
    }

    async fn read_utf_with_len(&mut self, max_length: u32) -> Result<String, String> {
        let (length, _length_varint_length) = self.read_varint().await?;
        // i don't know why it's multiplied by 4 but it's like that in mojang's code so
        if length < 0 {
            return Err(
                "The received encoded string buffer length is less than zero! Weird string!"
                    .to_string(),
            );
        }
        if length as u32 > max_length * 4 {
            return Err(format!(
                "The received encoded string buffer length is longer than maximum allowed ({} > {})",
                length,
                max_length * 4
            ));
        }

        // this is probably quite inefficient, idk how to do it better
        let mut string = String::new();
        let mut buffer = vec![0; length as usize];
        self.read_exact(&mut buffer)
            .await
            .map_err(|_| "Invalid UTF-8".to_string())?;

        string.push_str(std::str::from_utf8(&buffer).unwrap());
        if string.len() > length as usize {
            return Err(format!(
                "The received string length is longer than maximum allowed ({} > {})",
                length, max_length
            ));
        }

        Ok(string)
    }

    /// Read a single byte from the reader
    async fn read_byte(&mut self) -> Result<u8, String> {
        match AsyncReadExt::read_u8(self).await {
            Ok(r) => Ok(r),
            Err(_) => Err("Error reading byte".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use tokio::io::BufReader;

    #[test]
    fn test_write_varint() {
        let mut buf = Vec::new();
        write_varint(&mut buf, 123456);
        assert_eq!(buf, vec![192, 196, 7]);

        let mut buf = Vec::new();
        write_varint(&mut buf, 0);
        assert_eq!(buf, vec![0]);
    }

    #[tokio::test]
    async fn test_read_varint() {
        let mut buf = BufReader::new(Cursor::new(vec![192, 196, 7]));
        assert_eq!(buf.read_varint().await.unwrap(), (123456, 3));

        let mut buf = BufReader::new(Cursor::new(vec![0]));
        assert_eq!(buf.read_varint().await.unwrap(), (0, 1));

        let mut buf = BufReader::new(Cursor::new(vec![1]));
        assert_eq!(buf.read_varint().await.unwrap(), (1, 1));
    }

    #[tokio::test]
    async fn test_read_varint_longer() {
        let mut buf = BufReader::new(Cursor::new(vec![138, 56, 0, 135, 56, 123]));
        assert_eq!(buf.read_varint().await.unwrap(), (7178, 2));
    }
}
