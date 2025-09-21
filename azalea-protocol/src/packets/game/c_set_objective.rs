use std::io::{self, Cursor, Write};

use azalea_buf::{AzBuf, AzaleaRead, AzaleaWrite};
use azalea_chat::{FormattedText, numbers::NumberFormat};
use azalea_core::objectives::ObjectiveCriteria;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundSetObjective {
    pub objective_name: String,
    pub method: Method,
}

#[derive(Clone, Copy, Debug, AzBuf, PartialEq)]
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
        number_format: NumberFormat,
    },
    Remove,
    Change {
        display_name: FormattedText,
        render_type: ObjectiveCriteria,
        number_format: NumberFormat,
    },
}

impl AzaleaRead for Method {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, azalea_buf::BufReadError> {
        let kind = MethodKind::azalea_read(buf)?;
        match kind {
            MethodKind::Add => Ok(Method::Add {
                display_name: FormattedText::azalea_read(buf)?,
                render_type: ObjectiveCriteria::azalea_read(buf)?,
                number_format: NumberFormat::azalea_read(buf)?,
            }),
            MethodKind::Remove => Ok(Method::Remove),
            MethodKind::Change => Ok(Method::Change {
                display_name: FormattedText::azalea_read(buf)?,
                render_type: ObjectiveCriteria::azalea_read(buf)?,
                number_format: NumberFormat::azalea_read(buf)?,
            }),
        }
    }
}

impl AzaleaWrite for Method {
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        match self {
            Method::Add {
                display_name,
                render_type,
                number_format,
            } => {
                MethodKind::Add.azalea_write(buf)?;
                display_name.azalea_write(buf)?;
                render_type.azalea_write(buf)?;
                number_format.azalea_write(buf)?;
            }
            Method::Remove => MethodKind::Remove.azalea_write(buf)?,
            Method::Change {
                display_name,
                render_type,
                number_format,
            } => {
                MethodKind::Change.azalea_write(buf)?;
                display_name.azalea_write(buf)?;
                render_type.azalea_write(buf)?;
                number_format.azalea_write(buf)?;
            }
        }
        Ok(())
    }
}
