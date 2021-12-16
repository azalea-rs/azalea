//! Utilities for reading and writing for the Minecraft protocol

use std::{future::Future, io::Write};

use async_trait::async_trait;
use byteorder::{BigEndian, WriteBytesExt};
use tokio::io::{AsyncRead, AsyncReadExt};

// const DEFAULT_NBT_QUOTA: u32 = 2097152;
const MAX_STRING_LENGTH: u16 = 32767;
// const MAX_COMPONENT_STRING_LENGTH: u32 = 262144;

#[async_trait]
pub trait Writable {
    fn write_collection<F, T>(
        &mut self,
        collection: Vec<T>,
        writer: F,
    ) -> Result<(), std::io::Error>
    where
        F: FnOnce(&mut Self, T) -> Result<(), std::io::Error> + Copy,
        Self: Sized;
    fn write_byte(&mut self, n: u8) -> Result<(), std::io::Error>;
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), std::io::Error>;
    fn write_varint(&mut self, value: i32) -> Result<(), std::io::Error>;
    fn write_utf_with_len(&mut self, string: &str, len: usize) -> Result<(), std::io::Error>;
    fn write_utf(&mut self, string: &str) -> Result<(), std::io::Error>;
    fn write_short(&mut self, n: u16) -> Result<(), std::io::Error>;
    fn write_byte_array(&mut self, bytes: &[u8]) -> Result<(), std::io::Error>;
}

#[async_trait]
impl Writable for Vec<u8> {
    fn write_collection<F, T>(
        &mut self,
        collection: Vec<T>,
        writer: F,
    ) -> Result<(), std::io::Error>
    where
        F: FnOnce(&mut Self, T) -> Result<(), std::io::Error> + Copy,
        Self: Sized,
    {
        self.write_varint(collection.len() as i32)?;
        for item in collection {
            writer(self, item)?;
        }
        Ok(())
    }

    fn write_byte(&mut self, n: u8) -> Result<(), std::io::Error> {
        WriteBytesExt::write_u8(self, n)
    }

    fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), std::io::Error> {
        Ok(self.extend_from_slice(bytes))
    }

    fn write_varint(&mut self, mut value: i32) -> Result<(), std::io::Error> {
        let mut buffer = [0];
        if value == 0 {
            self.write_all(&buffer).unwrap();
        }
        while value != 0 {
            buffer[0] = (value & 0b0111_1111) as u8;
            value = (value >> 7) & (i32::max_value() >> 6);
            if value != 0 {
                buffer[0] |= 0b1000_0000;
            }
            self.write_all(&buffer)?;
        }
        Ok(())
    }

    fn write_utf_with_len(&mut self, string: &str, len: usize) -> Result<(), std::io::Error> {
        if string.len() > len {
            panic!(
                "String too big (was {} bytes encoded, max {})",
                string.len(),
                len
            );
        }
        self.write_varint(string.len() as i32);
        self.write_bytes(string.as_bytes())
    }

    fn write_utf(&mut self, string: &str) -> Result<(), std::io::Error> {
        self.write_utf_with_len(string, MAX_STRING_LENGTH.into())
    }

    fn write_short(&mut self, n: u16) -> Result<(), std::io::Error> {
        WriteBytesExt::write_u16::<BigEndian>(self, n)
    }

    fn write_byte_array(&mut self, bytes: &[u8]) -> Result<(), std::io::Error> {
        self.write_varint(bytes.len() as i32);
        self.write_bytes(bytes)
    }
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
        buf.write_varint(123456).unwrap();
        assert_eq!(buf, vec![192, 196, 7]);

        let mut buf = Vec::new();
        buf.write_varint(0).unwrap();
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
    #[tokio::test]
    async fn test_collection() {
        let mut buf = Vec::new();
        buf.write_collection(vec!["a", "bc", "def"], Vec::write_utf)
            .unwrap();

        // there's no read_collection because idk how to do it in rust
        let mut buf = BufReader::new(Cursor::new(buf));

        let mut result = Vec::new();
        let length = buf.read_varint().await.unwrap().0;
        for _ in 0..length {
            result.push(buf.read_utf().await.unwrap());
        }

        assert_eq!(result, vec!["a", "bc", "def"]);
    }
}
