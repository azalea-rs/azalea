use super::{UnsizedByteArray, MAX_STRING_LENGTH};
use byteorder::{ReadBytesExt, BE};
use log::warn;
use std::{
    backtrace::Backtrace,
    collections::HashMap,
    hash::Hash,
    io::{Cursor, Read},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BufReadError {
    #[error("Invalid VarInt")]
    InvalidVarInt,
    #[error("Invalid VarLong")]
    InvalidVarLong,
    #[error("Error reading bytes")]
    CouldNotReadBytes,
    #[error("The received encoded string buffer length is longer than maximum allowed ({length} > {max_length})")]
    StringLengthTooLong { length: u32, max_length: u32 },
    #[error("{source}")]
    Io {
        #[from]
        #[backtrace]
        source: std::io::Error,
    },
    #[error("Invalid UTF-8: {bytes:?} (lossy: {lossy:?})")]
    InvalidUtf8 {
        bytes: Vec<u8>,
        lossy: String,
        // backtrace: Backtrace,
    },
    #[error("Unexpected enum variant {id}")]
    UnexpectedEnumVariant { id: i32 },
    #[error("Unexpected enum variant {id}")]
    UnexpectedStringEnumVariant { id: String },
    #[error("Tried to read {attempted_read} bytes but there were only {actual_read}")]
    UnexpectedEof {
        attempted_read: usize,
        actual_read: usize,
        backtrace: Backtrace,
    },
    #[error("{0}")]
    Custom(String),
    #[cfg(feature = "serde_json")]
    #[error("{source}")]
    Deserialization {
        #[from]
        #[backtrace]
        source: serde_json::Error,
    },
}

fn read_bytes<'a>(buf: &'a mut Cursor<&[u8]>, length: usize) -> Result<&'a [u8], BufReadError> {
    if length > (buf.get_ref().len() - buf.position() as usize) {
        return Err(BufReadError::UnexpectedEof {
            attempted_read: length,
            actual_read: buf.get_ref().len() - buf.position() as usize,
            backtrace: Backtrace::capture(),
        });
    }
    let initial_position = buf.position() as usize;
    buf.set_position(buf.position() + length as u64);
    let data = &buf.get_ref()[initial_position..initial_position + length];
    Ok(data)
}

fn read_utf_with_len(buf: &mut Cursor<&[u8]>, max_length: u32) -> Result<String, BufReadError> {
    let length = u32::var_read_from(buf)?;
    // i don't know why it's multiplied by 4 but it's like that in mojang's code so
    if length > max_length * 4 {
        return Err(BufReadError::StringLengthTooLong {
            length,
            max_length: max_length * 4,
        });
    }

    let buffer = read_bytes(buf, length as usize)?;
    let string = std::str::from_utf8(buffer)
        .map_err(|_| BufReadError::InvalidUtf8 {
            bytes: buffer.to_vec(),
            lossy: String::from_utf8_lossy(buffer).to_string(),
            // backtrace: Backtrace::capture(),
        })?
        .to_string();
    if string.len() > length as usize {
        return Err(BufReadError::StringLengthTooLong { length, max_length });
    }

    Ok(string)
}

// fast varints modified from https://github.com/luojia65/mc-varint/blob/master/src/lib.rs#L67
/// Read a single varint from the reader and return the value, along with the
/// number of bytes read
// pub async fn read_varint_async(
//     reader: &mut (dyn AsyncRead + Unpin + Send),
// ) -> Result<i32, BufReadError> {
//     let mut buffer = [0];
//     let mut ans = 0;
//     for i in 0..5 {
//         reader.read_exact(&mut buffer).await?;
//         ans |= ((buffer[0] & 0b0111_1111) as i32) << (7 * i);
//         if buffer[0] & 0b1000_0000 == 0 {
//             break;
//         }
//     }
//     Ok(ans)
// }

pub trait McBufReadable
where
    Self: Sized,
{
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError>;
}

pub trait McBufVarReadable
where
    Self: Sized,
{
    fn var_read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError>;
}

impl McBufReadable for i32 {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(buf.read_i32::<BE>()?)
    }
}

impl McBufVarReadable for i32 {
    // fast varints modified from https://github.com/luojia65/mc-varint/blob/master/src/lib.rs#L67
    /// Read a single varint from the reader and return the value
    fn var_read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let mut buffer = [0];
        let mut ans = 0;
        for i in 0..5 {
            buf.read_exact(&mut buffer)?;
            ans |= ((buffer[0] & 0b0111_1111) as i32) << (7 * i);
            if buffer[0] & 0b1000_0000 == 0 {
                break;
            }
        }
        Ok(ans)
    }
}

impl McBufVarReadable for i64 {
    // fast varints modified from https://github.com/luojia65/mc-varint/blob/master/src/lib.rs#L54
    fn var_read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let mut buffer = [0];
        let mut ans = 0;
        for i in 0..8 {
            buf.read_exact(&mut buffer)
                .map_err(|_| BufReadError::InvalidVarLong)?;
            ans |= ((buffer[0] & 0b0111_1111) as i64) << (7 * i);
            if buffer[0] & 0b1000_0000 == 0 {
                break;
            }
        }
        Ok(ans)
    }
}
impl McBufVarReadable for u64 {
    fn var_read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        i64::var_read_from(buf).map(|i| i as u64)
    }
}

impl McBufReadable for UnsizedByteArray {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        // read to end of the buffer
        let data = buf.get_ref()[buf.position() as usize..].to_vec();
        buf.set_position((buf.position()) + data.len() as u64);
        Ok(UnsizedByteArray(data))
    }
}

impl<T: McBufReadable + Send> McBufReadable for Vec<T> {
    default fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let length = u32::var_read_from(buf)? as usize;
        // we don't set the capacity here so we can't get exploited into
        // allocating a bunch
        let mut contents = vec![];
        for _ in 0..length {
            contents.push(T::read_from(buf)?);
        }
        Ok(contents)
    }
}

impl<K: McBufReadable + Send + Eq + Hash, V: McBufReadable + Send> McBufReadable for HashMap<K, V> {
    default fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let length = i32::var_read_from(buf)? as usize;
        let mut contents = HashMap::new();
        for _ in 0..length {
            contents.insert(K::read_from(buf)?, V::read_from(buf)?);
        }
        Ok(contents)
    }
}

impl<K: McBufReadable + Send + Eq + Hash, V: McBufVarReadable + Send> McBufVarReadable
    for HashMap<K, V>
{
    default fn var_read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let length = i32::var_read_from(buf)? as usize;
        let mut contents = HashMap::new();
        for _ in 0..length {
            contents.insert(K::read_from(buf)?, V::var_read_from(buf)?);
        }
        Ok(contents)
    }
}

impl McBufReadable for Vec<u8> {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let length = i32::var_read_from(buf)? as usize;
        read_bytes(buf, length).map(|b| b.to_vec())
    }
}

impl McBufReadable for String {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        read_utf_with_len(buf, MAX_STRING_LENGTH.into())
    }
}

impl McBufReadable for u32 {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(i32::read_from(buf)? as u32)
    }
}

impl McBufVarReadable for u32 {
    fn var_read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(i32::var_read_from(buf)? as u32)
    }
}

impl McBufReadable for u16 {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        i16::read_from(buf).map(|i| i as u16)
    }
}

impl McBufReadable for i16 {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(buf.read_i16::<BE>()?)
    }
}

impl McBufVarReadable for u16 {
    fn var_read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(i32::var_read_from(buf)? as u16)
    }
}

impl<T: McBufVarReadable> McBufVarReadable for Vec<T> {
    fn var_read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let length = i32::var_read_from(buf)? as usize;
        let mut contents = Vec::new();
        for _ in 0..length {
            contents.push(T::var_read_from(buf)?);
        }
        Ok(contents)
    }
}

impl McBufReadable for i64 {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(buf.read_i64::<BE>()?)
    }
}

impl McBufReadable for u64 {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        i64::read_from(buf).map(|i| i as u64)
    }
}

impl McBufReadable for bool {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let byte = u8::read_from(buf)?;
        if byte > 1 {
            warn!("Boolean value was not 0 or 1, but {}", byte);
        }
        Ok(byte != 0)
    }
}

impl McBufReadable for u8 {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(buf.read_u8()?)
    }
}

impl McBufReadable for i8 {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        u8::read_from(buf).map(|i| i as i8)
    }
}

impl McBufReadable for f32 {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(buf.read_f32::<BE>()?)
    }
}

impl McBufReadable for f64 {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(buf.read_f64::<BE>()?)
    }
}

impl<T: McBufReadable> McBufReadable for Option<T> {
    default fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let present = bool::read_from(buf)?;
        Ok(if present {
            Some(T::read_from(buf)?)
        } else {
            None
        })
    }
}

impl<T: McBufVarReadable> McBufVarReadable for Option<T> {
    default fn var_read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let present = bool::read_from(buf)?;
        Ok(if present {
            Some(T::var_read_from(buf)?)
        } else {
            None
        })
    }
}

// [String; 4]
impl<T: McBufReadable, const N: usize> McBufReadable for [T; N] {
    default fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let mut contents = Vec::with_capacity(N);
        for _ in 0..N {
            contents.push(T::read_from(buf)?);
        }
        contents.try_into().map_err(|_| {
            unreachable!("Panic is not possible since the Vec is the same size as the array")
        })
    }
}
