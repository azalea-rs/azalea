use async_trait::async_trait;
use azalea_core::resource_location::ResourceLocation;
use tokio::io::{AsyncRead, AsyncReadExt};

use super::MAX_STRING_LENGTH;

#[async_trait]
pub trait Readable {
    async fn read_int_id_list(&mut self) -> Result<Vec<i32>, String>;
    async fn read_varint(&mut self) -> Result<i32, String>;
    fn get_varint_size(&mut self, value: i32) -> u8;
    fn get_varlong_size(&mut self, value: i32) -> u8;
    async fn read_byte_array(&mut self) -> Result<Vec<u8>, String>;
    async fn read_bytes_with_len(&mut self, n: usize) -> Result<Vec<u8>, String>;
    async fn read_bytes(&mut self) -> Result<Vec<u8>, String>;
    async fn read_utf(&mut self) -> Result<String, String>;
    async fn read_utf_with_len(&mut self, max_length: u32) -> Result<String, String>;
    async fn read_byte(&mut self) -> Result<u8, String>;
    async fn read_int(&mut self) -> Result<i32, String>;
    async fn read_boolean(&mut self) -> Result<bool, String>;
    async fn read_nbt(&mut self) -> Result<azalea_nbt::Tag, String>;
    async fn read_long(&mut self) -> Result<i64, String>;
    async fn read_resource_location(&mut self) -> Result<ResourceLocation, String>;
    async fn read_short(&mut self) -> Result<i16, String>;
}

#[async_trait]
impl<R> Readable for R
where
    R: AsyncRead + std::marker::Unpin + std::marker::Send,
{
    async fn read_int_id_list(&mut self) -> Result<Vec<i32>, String> {
        let len = self.read_varint().await?;
        let mut list = Vec::with_capacity(len as usize);
        for _ in 0..len {
            list.push(self.read_varint().await?);
        }
        Ok(list)
    }

    // fast varints stolen from https://github.com/luojia65/mc-varint/blob/master/src/lib.rs#L67
    /// Read a single varint from the reader and return the value, along with the number of bytes read
    async fn read_varint(&mut self) -> Result<i32, String> {
        let mut buffer = [0];
        let mut ans = 0;
        for i in 0..4 {
            self.read_exact(&mut buffer)
                .await
                .map_err(|_| "Invalid VarInt".to_string())?;
            ans |= ((buffer[0] & 0b0111_1111) as i32) << (7 * i);
            if buffer[0] & 0b1000_0000 == 0 {
                return Ok(ans);
            }
        }
        Ok(ans)
    }

    fn get_varint_size(&mut self, value: i32) -> u8 {
        for i in 1..5 {
            if (value & -1 << (i * 7)) != 0 {
                continue;
            }
            return i;
        }
        5
    }

    fn get_varlong_size(&mut self, value: i32) -> u8 {
        for i in 1..10 {
            if (value & -1 << (i * 7)) != 0 {
                continue;
            }
            return i;
        }
        10
    }

    async fn read_byte_array(&mut self) -> Result<Vec<u8>, String> {
        let length = self.read_varint().await? as usize;
        Ok(self.read_bytes_with_len(length).await?)
    }

    async fn read_bytes_with_len(&mut self, n: usize) -> Result<Vec<u8>, String> {
        let mut bytes = vec![0; n];
        match AsyncReadExt::read_exact(self, &mut bytes).await {
            Ok(_) => Ok(bytes),
            Err(_) => Err("Error reading bytes".to_string()),
        }
    }

    async fn read_bytes(&mut self) -> Result<Vec<u8>, String> {
        // read to end of the buffer
        let mut bytes = vec![];
        AsyncReadExt::read_to_end(self, &mut bytes)
            .await
            .map_err(|_| "Error reading bytes".to_string())?;
        Ok(bytes)
    }

    async fn read_utf(&mut self) -> Result<String, String> {
        self.read_utf_with_len(MAX_STRING_LENGTH.into()).await
    }

    async fn read_utf_with_len(&mut self, max_length: u32) -> Result<String, String> {
        let length = self.read_varint().await?;
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

    async fn read_int(&mut self) -> Result<i32, String> {
        match AsyncReadExt::read_i32(self).await {
            Ok(r) => Ok(r),
            Err(_) => Err("Error reading int".to_string()),
        }
    }

    async fn read_boolean(&mut self) -> Result<bool, String> {
        match self.read_byte().await {
            Ok(0) => Ok(false),
            Ok(1) => Ok(true),
            _ => Err("Error reading boolean".to_string()),
        }
    }

    async fn read_nbt(&mut self) -> Result<azalea_nbt::Tag, String> {
        match azalea_nbt::Tag::read(self).await {
            Ok(r) => Ok(r),
            // Err(e) => Err(e.to_string()),
            Err(e) => Err(e.to_string()).unwrap(),
        }
    }

    async fn read_long(&mut self) -> Result<i64, String> {
        match AsyncReadExt::read_i64(self).await {
            Ok(r) => Ok(r),
            Err(_) => Err("Error reading long".to_string()),
        }
    }

    async fn read_resource_location(&mut self) -> Result<ResourceLocation, String> {
        // get the resource location from the string
        let location_string = self.read_utf().await?;
        let location = ResourceLocation::new(&location_string)?;
        Ok(location)
    }

    async fn read_short(&mut self) -> Result<i16, String> {
        match AsyncReadExt::read_i16(self).await {
            Ok(r) => Ok(r),
            Err(_) => Err("Error reading short".to_string()),
        }
    }
}

#[async_trait]
pub trait McBufReadable
where
    Self: Sized,
{
    async fn read_into<R>(buf: &mut R) -> Result<Self, String>
    where
        R: AsyncRead + std::marker::Unpin + std::marker::Send;
}

#[async_trait]
pub trait McBufVarintReadable
where
    Self: Sized,
{
    async fn varint_read_into<R>(buf: &mut R) -> Result<Self, String>
    where
        R: AsyncRead + std::marker::Unpin + std::marker::Send;
}

#[async_trait]
impl McBufReadable for i32 {
    async fn read_into<R>(buf: &mut R) -> Result<Self, String>
    where
        R: AsyncRead + std::marker::Unpin + std::marker::Send,
    {
        buf.read_int().await
    }
}

#[async_trait]
impl McBufVarintReadable for i32 {
    async fn varint_read_into<R>(buf: &mut R) -> Result<Self, String>
    where
        R: AsyncRead + std::marker::Unpin + std::marker::Send,
    {
        buf.read_varint().await
    }
}

#[async_trait]
impl McBufReadable for Vec<u8> {
    async fn read_into<R>(buf: &mut R) -> Result<Self, String>
    where
        R: AsyncRead + std::marker::Unpin + std::marker::Send,
    {
        buf.read_bytes().await
    }
}

// string
#[async_trait]
impl McBufReadable for String {
    async fn read_into<R>(buf: &mut R) -> Result<Self, String>
    where
        R: AsyncRead + std::marker::Unpin + std::marker::Send,
    {
        buf.read_utf().await
    }
}

// ResourceLocation
#[async_trait]
impl McBufReadable for ResourceLocation {
    async fn read_into<R>(buf: &mut R) -> Result<Self, String>
    where
        R: AsyncRead + std::marker::Unpin + std::marker::Send,
    {
        buf.read_resource_location().await
    }
}

// u32
#[async_trait]
impl McBufReadable for u32 {
    async fn read_into<R>(buf: &mut R) -> Result<Self, String>
    where
        R: AsyncRead + std::marker::Unpin + std::marker::Send,
    {
        buf.read_int().await.map(|i| i as u32)
    }
}

// u32 varint
#[async_trait]
impl McBufVarintReadable for u32 {
    async fn varint_read_into<R>(buf: &mut R) -> Result<Self, String>
    where
        R: AsyncRead + std::marker::Unpin + std::marker::Send,
    {
        buf.read_varint().await.map(|i| i as u32)
    }
}

// u16
#[async_trait]
impl McBufReadable for u16 {
    async fn read_into<R>(buf: &mut R) -> Result<Self, String>
    where
        R: AsyncRead + std::marker::Unpin + std::marker::Send,
    {
        buf.read_short().await.map(|i| i as u16)
    }
}

// u16 varint
#[async_trait]
impl McBufVarintReadable for u16 {
    async fn varint_read_into<R>(buf: &mut R) -> Result<Self, String>
    where
        R: AsyncRead + std::marker::Unpin + std::marker::Send,
    {
        buf.read_varint().await.map(|i| i as u16)
    }
}
