use std::{
    fmt::Debug,
    io::{self, Cursor, Write},
};

use azalea_buf::{AzBuf, AzBufLimited, AzBufVar};

/// Used for written books.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Filterable<T> {
    pub raw: T,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub filtered: Option<T>,
}

impl<T: AzBuf> AzBuf for Filterable<T> {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, azalea_buf::BufReadError> {
        let raw = AzBuf::azalea_read(buf)?;
        let filtered = AzBuf::azalea_read(buf)?;
        Ok(Self { raw, filtered })
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        self.raw.azalea_write(buf)?;
        self.filtered.azalea_write(buf)?;
        Ok(())
    }
}
impl<T: AzBufLimited> AzBufLimited for Filterable<T> {
    fn azalea_read_limited(
        buf: &mut Cursor<&[u8]>,
        limit: u32,
    ) -> Result<Self, azalea_buf::BufReadError> {
        let raw = AzBufLimited::azalea_read_limited(buf, limit)?;
        let filtered = AzBufLimited::azalea_read_limited(buf, limit)?;
        Ok(Self { raw, filtered })
    }
}
impl<T: AzBufVar> AzBufVar for Filterable<T> {
    fn azalea_read_var(buf: &mut Cursor<&[u8]>) -> Result<Self, azalea_buf::BufReadError> {
        let raw = AzBufVar::azalea_read_var(buf)?;
        let filtered = AzBufVar::azalea_read_var(buf)?;
        Ok(Self { raw, filtered })
    }
    fn azalea_write_var(&self, buf: &mut impl Write) -> io::Result<()> {
        self.raw.azalea_write_var(buf)?;
        self.filtered.azalea_write_var(buf)?;
        Ok(())
    }
}

impl<T: Clone> Clone for Filterable<T> {
    fn clone(&self) -> Self {
        Self {
            raw: self.raw.clone(),
            filtered: self.filtered.clone(),
        }
    }
}
impl<T: PartialEq> PartialEq for Filterable<T> {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw && self.filtered == other.filtered
    }
}
impl<T: Debug> Debug for Filterable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Filterable")
            .field("raw", &self.raw)
            .field("filtered", &self.filtered)
            .finish()
    }
}
