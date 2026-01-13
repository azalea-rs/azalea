mod extra;
mod primitives;

use std::{
    backtrace::Backtrace,
    io::{self, Cursor, Write},
};

use thiserror::Error;

/// A trait that's implemented on types that are used by the Minecraft protocol.
pub trait AzBuf
where
    Self: Sized,
{
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError>;
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()>;
}

/// Used for types that have an alternative variable-length encoding.
///
/// This mostly exists for varints.
pub trait AzBufVar
where
    Self: Sized,
{
    fn azalea_read_var(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError>;
    fn azalea_write_var(&self, buf: &mut impl Write) -> io::Result<()>;
}

/// Used for types that have some configurable limit.
///
/// For example, the implementation of this on `String` limits the maximum
/// length of the string.
///
/// This exists partially as an anti-abuse mechanism in Minecraft, so there is
/// no limited write function.
pub trait AzBufLimited
where
    Self: Sized,
{
    fn azalea_read_limited(buf: &mut Cursor<&[u8]>, limit: u32) -> Result<Self, BufReadError>;
}

#[derive(Debug, Error)]
pub enum BufReadError {
    #[error("Invalid VarInt")]
    InvalidVarInt,
    #[error("Invalid VarLong")]
    InvalidVarLong,
    #[error("Error reading bytes")]
    CouldNotReadBytes,
    #[error(
        "The received encoded string buffer length is longer than maximum allowed ({length} > {max_length})"
    )]
    StringLengthTooLong { length: u32, max_length: u32 },
    #[error("The received Vec length is longer than maximum allowed ({length} > {max_length})")]
    VecLengthTooLong { length: u32, max_length: u32 },
    #[error("{source}")]
    Io {
        #[from]
        #[backtrace]
        source: io::Error,
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
    #[error("{source}")]
    Nbt {
        #[from]
        #[backtrace]
        source: simdnbt::Error,
    },
    #[error("{source}")]
    DeserializeNbt {
        #[from]
        #[backtrace]
        source: simdnbt::DeserializeError,
    },
}

pub(crate) fn read_bytes<'a>(
    buf: &'a mut Cursor<&[u8]>,
    length: usize,
) -> Result<&'a [u8], BufReadError> {
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

pub(crate) fn read_utf_with_len<'a>(
    buf: &'a mut Cursor<&[u8]>,
    max_length: u32,
) -> Result<&'a str, BufReadError> {
    let length = u32::azalea_read_var(buf)?;
    // i don't know why it's multiplied by 4 but it's like that in mojang's code so
    if length > max_length * 4 {
        return Err(BufReadError::StringLengthTooLong {
            length,
            max_length: max_length * 4,
        });
    }

    let buffer = read_bytes(buf, length as usize)?;
    let string = std::str::from_utf8(buffer).map_err(|_| BufReadError::InvalidUtf8 {
        bytes: buffer.to_vec(),
        lossy: String::from_utf8_lossy(buffer).to_string(),
        // backtrace: Backtrace::capture(),
    })?;
    if string.len() > length as usize {
        return Err(BufReadError::StringLengthTooLong { length, max_length });
    }

    Ok(string)
}

pub(crate) fn write_utf_with_len(
    buf: &mut impl Write,
    string: &str,
    max_len: u32,
) -> io::Result<()> {
    let actual_len = string.len();
    if actual_len > max_len as usize {
        panic!("String too big (was {actual_len} bytes encoded, max {max_len})");
    }
    string.as_bytes().to_vec().azalea_write(buf)?;
    Ok(())
}
