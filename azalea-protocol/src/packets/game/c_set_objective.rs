use std::io::{self, Cursor, Write};

use azalea_buf::AzBuf;
use azalea_chat::FormattedText;
use azalea_core::objectives::ObjectiveCriteria;
use azalea_protocol_macros::ClientboundGamePacket;

use crate::common::number_formatting::NumberFormat;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundSetObjective {
    pub objective_name: String,
    pub method: Method,
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq)]
pub enum MethodKind {
    Add,
    Remove,
    Change,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Method {
    Add {
        display_name: FormattedText,
        render_type: ObjectiveCriteria,
        number_format: Option<NumberFormat>,
    },
    Remove,
    Change {
        display_name: FormattedText,
        render_type: ObjectiveCriteria,
        number_format: Option<NumberFormat>,
    },
}

impl AzBuf for Method {
    fn azalea_read(
        ctx: azalea_buf::Ctx,
        buf: &mut Cursor<&[u8]>,
    ) -> Result<Self, azalea_buf::BufReadError> {
        let kind = MethodKind::azalea_read(ctx, buf)?;
        match kind {
            MethodKind::Add => Ok(Method::Add {
                display_name: FormattedText::azalea_read(ctx, buf)?,
                render_type: ObjectiveCriteria::azalea_read(ctx, buf)?,
                number_format: Option::<NumberFormat>::azalea_read(ctx, buf)?,
            }),
            MethodKind::Remove => Ok(Method::Remove),
            MethodKind::Change => Ok(Method::Change {
                display_name: FormattedText::azalea_read(ctx, buf)?,
                render_type: ObjectiveCriteria::azalea_read(ctx, buf)?,
                number_format: Option::<NumberFormat>::azalea_read(ctx, buf)?,
            }),
        }
    }
    fn azalea_write(&self, ctx: azalea_buf::Ctx, buf: &mut impl Write) -> io::Result<()> {
        match self {
            Method::Add {
                display_name,
                render_type,
                number_format,
            } => {
                MethodKind::Add.azalea_write(ctx, buf)?;
                display_name.azalea_write(ctx, buf)?;
                render_type.azalea_write(ctx, buf)?;
                number_format.azalea_write(ctx, buf)?;
            }
            Method::Remove => MethodKind::Remove.azalea_write(ctx, buf)?,
            Method::Change {
                display_name,
                render_type,
                number_format,
            } => {
                MethodKind::Change.azalea_write(ctx, buf)?;
                display_name.azalea_write(ctx, buf)?;
                render_type.azalea_write(ctx, buf)?;
                number_format.azalea_write(ctx, buf)?;
            }
        }
        Ok(())
    }
}
