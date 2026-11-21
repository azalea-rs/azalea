use std::{
    backtrace::Backtrace,
    fmt::{self, Debug},
};

use thiserror::Error;

pub type ReadResult<T> = std::result::Result<T, ReadError>;
pub type WriteResult<T> = std::result::Result<T, WriteError>;

#[derive(Debug)]
pub enum ReadErrorKind {
    Unimplemented,
    UnknownKey(Box<str>),
    MissingKey(&'static str),
    Custom(Box<str>),
}

#[derive(Debug, Error)]
#[error("{kind:?}")]
pub struct ReadError {
    pub kind: ReadErrorKind,
    pub backtrace: Backtrace,
}
impl ReadError {
    pub fn unimplemented() -> Self {
        Self {
            kind: ReadErrorKind::Unimplemented,
            backtrace: Backtrace::capture(),
        }
    }
    pub fn unknown_key(name: impl Into<Box<str>>) -> Self {
        Self {
            kind: ReadErrorKind::UnknownKey(name.into()),
            backtrace: Backtrace::capture(),
        }
    }
    pub fn missing_key(name: &'static str) -> Self {
        Self {
            kind: ReadErrorKind::MissingKey(name),
            backtrace: Backtrace::capture(),
        }
    }
    pub fn custom(message: impl Into<Box<str>>) -> Self {
        Self {
            kind: ReadErrorKind::Custom(message.into()),
            backtrace: Backtrace::capture(),
        }
    }
}
#[derive(Debug)]
pub enum WriteErrorKind {
    Unimplemented,
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
        Self {
            kind: WriteErrorKind::Unimplemented,
            backtrace: Backtrace::capture(),
        }
    }
    pub fn custom(message: impl Into<Box<str>>) -> Self {
        Self {
            kind: WriteErrorKind::Custom(message.into()),
            backtrace: Backtrace::capture(),
        }
    }
}
