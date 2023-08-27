use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid tag type: {0}")]
    InvalidTagType(u8),
    #[error("Invalid tag")]
    InvalidTag,
    #[error("Write error: {0}")]
    WriteError(#[from] std::io::Error),
    #[error("Utf8 error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("Unexpected EOF")]
    UnexpectedEof,
}
