use std::io::{self, Cursor, Write};

use azalea_buf::{AzBuf, BufReadError, BufReadErrorRepr};
use simdnbt::owned::NbtCompound;

// TODO: this should be moved into simdnbt
#[derive(Clone, Debug, PartialEq)]
pub struct OptionalNbtCompound(pub Option<NbtCompound>);

impl AzBuf for OptionalNbtCompound {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let tag_type = u8::azalea_read(buf)?;
        match tag_type {
            10 => Ok(Self(Some(
                simdnbt::owned::read_compound(buf).map_err(simdnbt::Error::from)?,
            ))),
            0 => Ok(Self(None)),
            _ => Err(BufReadErrorRepr::UnexpectedEnumVariant {
                id: tag_type.into(),
            }
            .into()),
        }
    }

    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        if let Some(compound) = &self.0 {
            10u8.azalea_write(buf)?;
            compound.azalea_write(buf)?;
        } else {
            0u8.azalea_write(buf)?;
        }
        Ok(())
    }
}
