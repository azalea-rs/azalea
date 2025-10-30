use std::{
    fmt::Debug,
    io::{self, Cursor, Write},
};

use azalea_buf::{AzaleaRead, AzaleaReadLimited, AzaleaReadVar, AzaleaWrite};
use serde::Serialize;

/// Used for written books.
#[derive(Serialize)]
pub struct Filterable<T> {
    pub raw: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filtered: Option<T>,
}

impl<T: AzaleaWrite> azalea_buf::AzaleaWrite for Filterable<T> {
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        self.raw.azalea_write(buf)?;
        self.filtered.azalea_write(buf)?;
        Ok(())
    }
}
impl<T: AzaleaRead> azalea_buf::AzaleaRead for Filterable<T> {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, azalea_buf::BufReadError> {
        let raw = AzaleaRead::azalea_read(buf)?;
        let filtered = AzaleaRead::azalea_read(buf)?;
        Ok(Self { raw, filtered })
    }
}
impl<T: AzaleaReadLimited> azalea_buf::AzaleaReadLimited for Filterable<T> {
    fn azalea_read_limited(
        buf: &mut Cursor<&[u8]>,
        limit: usize,
    ) -> Result<Self, azalea_buf::BufReadError> {
        let raw = AzaleaReadLimited::azalea_read_limited(buf, limit)?;
        let filtered = AzaleaReadLimited::azalea_read_limited(buf, limit)?;
        Ok(Self { raw, filtered })
    }
}
impl<T: AzaleaReadVar> azalea_buf::AzaleaReadVar for Filterable<T> {
    fn azalea_read_var(buf: &mut Cursor<&[u8]>) -> Result<Self, azalea_buf::BufReadError> {
        let raw = AzaleaReadVar::azalea_read_var(buf)?;
        let filtered = AzaleaReadVar::azalea_read_var(buf)?;
        Ok(Self { raw, filtered })
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
