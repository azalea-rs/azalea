use super::{UnsizedByteArray, MAX_STRING_LENGTH};
use byteorder::{ReadBytesExt, BE};
use std::{collections::HashMap, hash::Hash, io::Read};
use tokio::io::{AsyncRead, AsyncReadExt};

// TODO: get rid of Readable and use McBufReadable everywhere

pub trait Readable {
    fn read_int_id_list(&mut self) -> Result<Vec<i32>, String>;
    fn read_varint(&mut self) -> Result<i32, String>;
    fn get_varint_size(&mut self, value: i32) -> u8;
    fn get_varlong_size(&mut self, value: i32) -> u8;
    fn read_byte_array(&mut self) -> Result<Vec<u8>, String>;
    fn read_bytes_with_len(&mut self, n: usize) -> Result<Vec<u8>, String>;
    fn read_bytes(&mut self) -> Result<Vec<u8>, String>;
    fn read_utf(&mut self) -> Result<String, String>;
    fn read_utf_with_len(&mut self, max_length: u32) -> Result<String, String>;
    fn read_byte(&mut self) -> Result<u8, String>;
    fn read_int(&mut self) -> Result<i32, String>;
    fn read_boolean(&mut self) -> Result<bool, String>;
    fn read_long(&mut self) -> Result<i64, String>;
    fn read_short(&mut self) -> Result<i16, String>;
    fn read_float(&mut self) -> Result<f32, String>;
    fn read_double(&mut self) -> Result<f64, String>;
}

impl<R> Readable for R
where
    R: Read,
{
    fn read_int_id_list(&mut self) -> Result<Vec<i32>, String> {
        let len = self.read_varint()?;
        let mut list = Vec::with_capacity(len as usize);
        for _ in 0..len {
            list.push(self.read_varint()?);
        }
        Ok(list)
    }

    // fast varints modified from https://github.com/luojia65/mc-varint/blob/master/src/lib.rs#L67
    /// Read a single varint from the reader and return the value, along with the number of bytes read
    fn read_varint(&mut self) -> Result<i32, String> {
        let mut buffer = [0];
        let mut ans = 0;
        for i in 0..5 {
            self.read_exact(&mut buffer)
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

    fn read_byte_array(&mut self) -> Result<Vec<u8>, String> {
        let length = self.read_varint()? as usize;
        self.read_bytes_with_len(length)
    }

    fn read_bytes_with_len(&mut self, n: usize) -> Result<Vec<u8>, String> {
        let mut buffer = vec![0; n];
        self.read_exact(&mut buffer)
            .map_err(|_| "Error reading bytes".to_string())?;
        Ok(buffer)
    }

    fn read_bytes(&mut self) -> Result<Vec<u8>, String> {
        // read to end of the buffer
        let mut bytes = vec![];
        self.read_to_end(&mut bytes)
            .map_err(|_| "Error reading bytes".to_string())?;
        Ok(bytes)
    }

    fn read_utf(&mut self) -> Result<String, String> {
        self.read_utf_with_len(MAX_STRING_LENGTH.into())
    }

    fn read_utf_with_len(&mut self, max_length: u32) -> Result<String, String> {
        let length = self.read_varint()?;
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
    fn read_byte(&mut self) -> Result<u8, String> {
        self.read_u8().map_err(|_| "Error reading byte".to_string())
    }

    fn read_int(&mut self) -> Result<i32, String> {
        match self.read_i32::<BE>() {
            Ok(r) => Ok(r),
            Err(_) => Err("Error reading int".to_string()),
        }
    }

    fn read_boolean(&mut self) -> Result<bool, String> {
        match self.read_byte()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err("Error reading boolean".to_string()),
        }
    }

    fn read_long(&mut self) -> Result<i64, String> {
        match self.read_i64::<BE>() {
            Ok(r) => Ok(r),
            Err(_) => Err("Error reading long".to_string()),
        }
    }

    fn read_short(&mut self) -> Result<i16, String> {
        match self.read_i16::<BE>() {
            Ok(r) => Ok(r),
            Err(_) => Err("Error reading short".to_string()),
        }
    }

    fn read_float(&mut self) -> Result<f32, String> {
        match self.read_f32::<BE>() {
            Ok(r) => Ok(r),
            Err(_) => Err("Error reading float".to_string()),
        }
    }

    fn read_double(&mut self) -> Result<f64, String> {
        match self.read_f64::<BE>() {
            Ok(r) => Ok(r),
            Err(_) => Err("Error reading double".to_string()),
        }
    }
}

// fast varints modified from https://github.com/luojia65/mc-varint/blob/master/src/lib.rs#L67
/// Read a single varint from the reader and return the value, along with the number of bytes read
pub async fn read_varint_async(reader: &mut (dyn AsyncRead + Unpin + Send)) -> Result<i32, String> {
    let mut buffer = [0];
    let mut ans = 0;
    for i in 0..5 {
        reader
            .read_exact(&mut buffer)
            .await
            .map_err(|_| "Invalid VarInt".to_string())?;
        ans |= ((buffer[0] & 0b0111_1111) as i32) << (7 * i);
        if buffer[0] & 0b1000_0000 == 0 {
            return Ok(ans);
        }
    }
    Ok(ans)
}

pub trait McBufReadable
where
    Self: Sized,
{
    fn read_into(buf: &mut impl Read) -> Result<Self, String>;
}

pub trait McBufVarReadable
where
    Self: Sized,
{
    fn var_read_into(buf: &mut impl Read) -> Result<Self, String>;
}

impl McBufReadable for i32 {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        Readable::read_int(buf)
    }
}

impl McBufVarReadable for i32 {
    fn var_read_into(buf: &mut impl Read) -> Result<Self, String> {
        buf.read_varint()
    }
}

impl McBufVarReadable for i64 {
    // fast varints modified from https://github.com/luojia65/mc-varint/blob/master/src/lib.rs#L54
    fn var_read_into(buf: &mut impl Read) -> Result<Self, String> {
        let mut buffer = [0];
        let mut ans = 0;
        for i in 0..8 {
            buf.read_exact(&mut buffer)
                .map_err(|_| "Invalid VarLong".to_string())?;
            ans |= ((buffer[0] & 0b0111_1111) as i64) << 7 * i;
            if buffer[0] & 0b1000_0000 == 0 {
                break;
            }
        }
        Ok(ans)
    }
}
impl McBufVarReadable for u64 {
    fn var_read_into(buf: &mut impl Read) -> Result<Self, String> {
        i64::var_read_into(buf).map(|i| i as u64)
    }
}

impl McBufReadable for UnsizedByteArray {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        Ok(buf.read_bytes()?.into())
    }
}

impl<T: McBufReadable + Send> McBufReadable for Vec<T> {
    default fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        let length = buf.read_varint()? as usize;
        let mut contents = Vec::with_capacity(length);
        for _ in 0..length {
            contents.push(T::read_into(buf)?);
        }
        Ok(contents)
    }
}

impl<K: McBufReadable + Send + Eq + Hash, V: McBufReadable + Send> McBufReadable for HashMap<K, V> {
    default fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        let length = buf.read_varint()? as usize;
        let mut contents = HashMap::with_capacity(length);
        for _ in 0..length {
            contents.insert(K::read_into(buf)?, V::read_into(buf)?);
        }
        Ok(contents)
    }
}

impl McBufReadable for Vec<u8> {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        buf.read_byte_array()
    }
}

impl McBufReadable for String {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        buf.read_utf()
    }
}

impl McBufReadable for u32 {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        Readable::read_int(buf).map(|i| i as u32)
    }
}

impl McBufVarReadable for u32 {
    fn var_read_into(buf: &mut impl Read) -> Result<Self, String> {
        buf.read_varint().map(|i| i as u32)
    }
}

impl McBufReadable for u16 {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        buf.read_short().map(|i| i as u16)
    }
}

impl McBufReadable for i16 {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        buf.read_short()
    }
}

impl McBufVarReadable for u16 {
    fn var_read_into(buf: &mut impl Read) -> Result<Self, String> {
        buf.read_varint().map(|i| i as u16)
    }
}

impl<T: McBufVarReadable> McBufVarReadable for Vec<T> {
    fn var_read_into(buf: &mut impl Read) -> Result<Self, String> {
        let length = buf.read_varint()? as usize;
        let mut contents = Vec::with_capacity(length);
        for _ in 0..length {
            contents.push(T::var_read_into(buf)?);
        }
        Ok(contents)
    }
}

impl McBufReadable for i64 {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        buf.read_long()
    }
}

impl McBufReadable for u64 {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        i64::read_into(buf).map(|i| i as u64)
    }
}

impl McBufReadable for bool {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        buf.read_boolean()
    }
}

impl McBufReadable for u8 {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        buf.read_byte()
    }
}

impl McBufReadable for i8 {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        buf.read_byte().map(|i| i as i8)
    }
}

impl McBufReadable for f32 {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        buf.read_float()
    }
}

impl McBufReadable for f64 {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        buf.read_double()
    }
}

impl<T: McBufReadable> McBufReadable for Option<T> {
    default fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        let present = buf.read_boolean()?;
        Ok(if present {
            Some(T::read_into(buf)?)
        } else {
            None
        })
    }
}
