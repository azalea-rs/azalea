use std::{
    backtrace::{Backtrace, BacktraceStatus},
    convert::Infallible,
    fmt::Debug,
};

use thiserror::Error;

use crate::number::Number;

pub type ReadResult<T> = std::result::Result<T, ReadError>;
pub type WriteResult<T> = std::result::Result<T, WriteError>;

#[derive(Debug)]
pub enum ReadErrorKind {
    Unimplemented,
    UnknownKey(Box<str>),
    MissingKey(&'static str),
    NoVariantsMatched(Box<[(&'static str, ReadError)]>),
    NumberTooBig(Number),
    Custom(Box<str>),
}

#[derive(Error)]
#[error("{kind:?}")]
pub struct ReadError {
    pub kind: ReadErrorKind,
    pub backtrace: Backtrace,
}
impl ReadError {
    pub fn unimplemented() -> Self {
        ReadErrorKind::Unimplemented.into()
    }
    pub fn unknown_key(name: impl Into<Box<str>>) -> Self {
        ReadErrorKind::UnknownKey(name.into()).into()
    }
    pub fn missing_key(name: &'static str) -> Self {
        ReadErrorKind::MissingKey(name).into()
    }
    pub fn no_variants_matched(errors: impl Into<Box<[(&'static str, ReadError)]>>) -> Self {
        ReadErrorKind::NoVariantsMatched(errors.into()).into()
    }
    pub fn number_too_big(n: Number) -> Self {
        ReadErrorKind::NumberTooBig(n).into()
    }
    pub fn custom(message: impl Into<Box<str>>) -> Self {
        ReadErrorKind::Custom(message.into()).into()
    }
}
impl From<ReadErrorKind> for ReadError {
    fn from(kind: ReadErrorKind) -> Self {
        Self {
            kind,
            backtrace: Backtrace::capture(),
        }
    }
}
impl From<Infallible> for ReadError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

impl Debug for ReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReadError")
            .field("kind", &self.kind)
            .finish()
            .and_then(|_| {
                if self.backtrace.status() == BacktraceStatus::Captured {
                    f.write_str(&format!("\n\x1b[90m{}\x1b[m", self.backtrace))
                } else {
                    f.write_str("")
                }
            })
    }
}

#[derive(Debug)]
pub enum WriteErrorKind {
    Unimplemented,
    NumberTooBig(u64),
    Custom(Box<str>),
}
#[derive(Debug, Error)]
#[error("{kind:?}")]
pub struct WriteError {
    pub kind: WriteErrorKind,
    pub backtrace: Backtrace,
}
impl WriteError {
    pub fn unimplemented() -> Self {
        WriteErrorKind::Unimplemented.into()
    }
    pub fn number_too_big(n: u64) -> Self {
        WriteErrorKind::NumberTooBig(n).into()
    }
    pub fn custom(message: impl Into<Box<str>>) -> Self {
        WriteErrorKind::Custom(message.into()).into()
    }
}
impl From<WriteErrorKind> for WriteError {
    fn from(kind: WriteErrorKind) -> Self {
        Self {
            kind,
            backtrace: Backtrace::capture(),
        }
    }
}
impl From<Infallible> for WriteError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}
