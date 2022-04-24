#[derive(Debug)]
pub enum Error {
    InvalidTagType(u8),
    InvalidTag,
    WriteError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InvalidTagType(id) => write!(f, "Invalid tag type: {}", id),
            Error::InvalidTag => write!(f, "Invalid tag"),
            Error::WriteError => write!(f, "Write error"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Error::WriteError
    }
}
impl From<std::string::FromUtf8Error> for Error {
    fn from(_: std::string::FromUtf8Error) -> Self {
        Error::WriteError
    }
}
