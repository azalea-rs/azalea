use std::backtrace::Backtrace;

pub enum ErrorKind {
    UnknownKey(Box<str>),
    MissingKey(&'static str),
    Custom(Box<str>),
}
pub struct Error {
    pub kind: ErrorKind,
    pub backtrace: Backtrace,
}
impl Error {
    pub fn unknown_key(name: impl Into<Box<str>>) -> Self {
        Self {
            kind: ErrorKind::UnknownKey(name.into()),
            backtrace: Backtrace::capture(),
        }
    }
    pub fn missing_key(name: &'static str) -> Self {
        Self {
            kind: ErrorKind::MissingKey(name),
            backtrace: Backtrace::capture(),
        }
    }
    pub fn custom(message: impl Into<Box<str>>) -> Self {
        Self {
            kind: ErrorKind::Custom(message.into()),
            backtrace: Backtrace::capture(),
        }
    }
}
