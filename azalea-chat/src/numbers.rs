//! Contains a few ways to style numbers. At the time of writing, Minecraft only
//! uses this for rendering scoreboard objectives.

use std::io::{Cursor, Write};

#[cfg(feature = "azalea-buf")]
use azalea_buf::{McBufReadable, McBufWritable};
use azalea_registry::NumberFormatKind;
use simdnbt::owned::Nbt;

use crate::FormattedText;

#[derive(Clone, Debug)]
pub enum NumberFormat {
    Blank,
    Styled { style: Nbt },
    Fixed { value: FormattedText },
}

#[cfg(feature = "azalea-buf")]
impl McBufReadable for NumberFormat {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, azalea_buf::BufReadError> {
        let kind = NumberFormatKind::read_from(buf)?;
        match kind {
            NumberFormatKind::Blank => Ok(NumberFormat::Blank),
            NumberFormatKind::Styled => Ok(NumberFormat::Styled {
                style: simdnbt::owned::read(buf)?,
            }),
            NumberFormatKind::Fixed => Ok(NumberFormat::Fixed {
                value: FormattedText::read_from(buf)?,
            }),
        }
    }
}

#[cfg(feature = "azalea-buf")]
impl McBufWritable for NumberFormat {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match self {
            NumberFormat::Blank => NumberFormatKind::Blank.write_into(buf)?,
            NumberFormat::Styled { style } => {
                NumberFormatKind::Styled.write_into(buf)?;
                style.write_into(buf)?;
            }
            NumberFormat::Fixed { value } => {
                NumberFormatKind::Fixed.write_into(buf)?;
                value.write_into(buf)?;
            }
        }
        Ok(())
    }
}
