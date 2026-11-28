//! Contains a few ways to style numbers. At the time of writing, Minecraft only
//! uses this for rendering scoreboard objectives.

use std::io::{self, Cursor, Write};

use azalea_buf::AzBuf;
use azalea_chat::FormattedText;
use azalea_registry::builtin::NumberFormatKind;
use simdnbt::owned::Nbt;

#[derive(Clone, Debug, PartialEq)]
pub enum NumberFormat {
    Blank,
    Styled { style: Nbt },
    Fixed { value: FormattedText },
}

impl AzBuf for NumberFormat {
    fn azalea_read(
        ctx: azalea_buf::Ctx,
        buf: &mut Cursor<&[u8]>,
    ) -> Result<Self, azalea_buf::BufReadError> {
        let kind = NumberFormatKind::azalea_read(ctx, buf)?;
        match kind {
            NumberFormatKind::Blank => Ok(NumberFormat::Blank),
            NumberFormatKind::Styled => Ok(NumberFormat::Styled {
                style: simdnbt::owned::read(buf)?,
            }),
            NumberFormatKind::Fixed => Ok(NumberFormat::Fixed {
                value: FormattedText::azalea_read(ctx, buf)?,
            }),
        }
    }
    fn azalea_write(&self, ctx: azalea_buf::Ctx, buf: &mut impl Write) -> io::Result<()> {
        match self {
            NumberFormat::Blank => NumberFormatKind::Blank.azalea_write(ctx, buf)?,
            NumberFormat::Styled { style } => {
                NumberFormatKind::Styled.azalea_write(ctx, buf)?;
                style.azalea_write(ctx, buf)?;
            }
            NumberFormat::Fixed { value } => {
                NumberFormatKind::Fixed.azalea_write(ctx, buf)?;
                value.azalea_write(ctx, buf)?;
            }
        }
        Ok(())
    }
}
