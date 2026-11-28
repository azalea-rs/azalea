use std::{
    backtrace::{Backtrace, BacktraceStatus},
    borrow::Cow,
    convert::Infallible,
    fmt::Debug,
    io,
};

use thiserror::Error;

use crate::number::Number;

pub type ReadResult<T> = std::result::Result<T, ReadError>;
pub type WriteResult<T> = std::result::Result<T, WriteError>;

#[derive(Debug, Clone)]
pub enum ReadErrorKind {
    Unimplemented,
    UnknownKey(Box<str>),
    MissingKey(&'static str),
    NoVariantsMatched(Box<[(&'static str, ReadError)]>),
    WhileReadingKey(Cow<'static, str>, Box<ReadErrorKind>),
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

    pub fn while_reading_key(mut self, key: impl Into<Cow<'static, str>>) -> Self {
        self.kind = ReadErrorKind::WhileReadingKey(key.into(), Box::new(self.kind));
        self
    }
}
impl ReadErrorKind {
    fn relevance(&self) -> usize {
        match self {
            ReadErrorKind::NoVariantsMatched(items) => items
                .iter()
                .map(|i| i.1.kind.relevance() + 1)
                .max()
                .unwrap_or_default(),
            ReadErrorKind::WhileReadingKey(_, e) => e.relevance() + 1,
            ReadErrorKind::MissingKey(_) => 1,
            _ => 10,
        }
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
impl Clone for ReadError {
    fn clone(&self) -> Self {
        Self {
            kind: self.kind.clone(),
            backtrace: Backtrace::capture(),
        }
    }
}

impl Debug for ReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pretty = Vec::new();
        let mut cur_kind = self.kind.clone();
        loop {
            cur_kind = match cur_kind {
                ReadErrorKind::NoVariantsMatched(items) => {
                    let most_relevant_item = items
                        .iter()
                        // rev to make sure we get the first one if there's a tie
                        .rev()
                        .max_by_key(|i| i.1.kind.relevance())
                        .unwrap()
                        .clone();
                    pretty.push(most_relevant_item.0.to_owned());
                    most_relevant_item.1.kind
                }
                ReadErrorKind::WhileReadingKey(k, e) => {
                    pretty.push(k.to_string());
                    *e
                }
                k => {
                    pretty.push(format!("{k:?}"));
                    break;
                }
            };
        }

        f.write_str("azalea_codec::error::ReadError: ")?;
        f.write_str(&pretty.join(" -> "))?;
        if self.backtrace.status() == BacktraceStatus::Captured {
            f.write_str(&format!("\n\x1b[90m{}\x1b[m", self.backtrace))?;
        }
        Ok(())
    }
}
impl From<ReadError> for io::Error {
    fn from(error: ReadError) -> Self {
        io::Error::other(error)
    }
}

#[derive(Debug, Clone)]
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
impl Clone for WriteError {
    fn clone(&self) -> Self {
        Self {
            kind: self.kind.clone(),
            backtrace: Backtrace::capture(),
        }
    }
}
impl From<WriteError> for io::Error {
    fn from(error: WriteError) -> Self {
        io::Error::other(error)
    }
}
