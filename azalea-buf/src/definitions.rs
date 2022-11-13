use std::ops::Deref;

/// A `Vec<u8>` that isn't prefixed by a VarInt with the size.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnsizedByteArray(pub Vec<u8>);

impl Deref for UnsizedByteArray {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<u8>> for UnsizedByteArray {
    fn from(vec: Vec<u8>) -> Self {
        Self(vec)
    }
}

impl From<&str> for UnsizedByteArray {
    fn from(s: &str) -> Self {
        Self(s.as_bytes().to_vec())
    }
}
