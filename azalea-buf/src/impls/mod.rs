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
pub enum BufReadErrorRepr {
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
        source: io::Error,
    },
    #[error("Invalid UTF-8: {bytes:?} (lossy: {lossy:?})")]
    InvalidUtf8 { bytes: Vec<u8>, lossy: String },
    #[error("Unexpected enum variant {id}")]
    UnexpectedEnumVariant { id: i32 },
    #[error("Unexpected enum variant {id}")]
    UnexpectedStringEnumVariant { id: String },
    #[error("Tried to read {attempted_read} bytes but there were only {actual_read}")]
    UnexpectedEof {
        attempted_read: usize,
        actual_read: usize,
    },
    #[error("{0}")]
    Custom(String),
    #[cfg(feature = "serde_json")]
    #[error("{source}")]
    Deserialization {
        #[from]
        source: serde_json::Error,
    },
    #[error("{source}")]
    Nbt {
        #[from]
        source: simdnbt::Error,
    },
    #[error("{source}")]
    DeserializeNbt {
        #[from]
        source: simdnbt::DeserializeError,
    },
}

// the only reason this is separated into a different type is for the backtrace
// impl
#[derive(Debug, Error)]
#[error("{source}")]
pub struct BufReadError {
    #[from]
    pub source: BufReadErrorRepr,
    #[backtrace]
    backtrace: Box<Backtrace>,
}
impl BufReadError {
    // maybe we should add more of these in the future
    pub fn custom(message: impl Into<String>) -> Self {
        // ^ this is intentionally Into<String> rather than ToString to avoid being too
        // permissive
        Self::from(BufReadErrorRepr::Custom(message.into()))
    }
}
macro_rules! buf_read_error_from_impl {
    ($t: ty) => {
        impl From<$t> for BufReadError {
            fn from(e: $t) -> Self {
                Self::from(BufReadErrorRepr::from(e))
            }
        }
    };
}
buf_read_error_from_impl!(io::Error);
buf_read_error_from_impl!(serde_json::Error);
buf_read_error_from_impl!(simdnbt::Error);
buf_read_error_from_impl!(simdnbt::DeserializeError);

pub(crate) fn read_bytes<'a>(
    buf: &'a mut Cursor<&[u8]>,
    length: usize,
) -> Result<&'a [u8], BufReadError> {
    if length > (buf.get_ref().len() - buf.position() as usize) {
        return Err(BufReadErrorRepr::UnexpectedEof {
            attempted_read: length,
            actual_read: buf.get_ref().len() - buf.position() as usize,
        }
        .into());
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
    // multiply the max by 4 because this length is for unicode codepoints
    if length > max_length * 4 {
        return Err(BufReadErrorRepr::StringLengthTooLong {
            length,
            max_length: max_length * 4,
        }
        .into());
    }

    let buffer = read_bytes(buf, length as usize)?;
    let string = std::str::from_utf8(buffer).map_err(|_| BufReadErrorRepr::InvalidUtf8 {
        bytes: buffer.to_vec(),
        lossy: String::from_utf8_lossy(buffer).to_string(),
    })?;
    if string.len() > length as usize {
        return Err(BufReadErrorRepr::StringLengthTooLong { length, max_length }.into());
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
