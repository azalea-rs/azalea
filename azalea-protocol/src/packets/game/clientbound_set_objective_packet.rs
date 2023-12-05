use std::io::{Cursor, Write};

use azalea_buf::{McBuf, McBufReadable, McBufWritable};
use azalea_chat::{numbers::NumberFormat, FormattedText};
use azalea_core::objectives::ObjectiveCriteria;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSetObjectivePacket {
    pub objective_name: String,
    pub method: Method,
}

#[derive(Clone, Copy, Debug, McBuf)]
pub enum MethodKind {
    Add,
    Remove,
    Change,
}

#[derive(Clone, Debug)]
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

impl McBufReadable for Method {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, azalea_buf::BufReadError> {
        let kind = MethodKind::read_from(buf)?;
        match kind {
            MethodKind::Add => Ok(Method::Add {
                display_name: FormattedText::read_from(buf)?,
                render_type: ObjectiveCriteria::read_from(buf)?,
                number_format: NumberFormat::read_from(buf)?,
            }),
            MethodKind::Remove => Ok(Method::Remove),
            MethodKind::Change => Ok(Method::Change {
                display_name: FormattedText::read_from(buf)?,
                render_type: ObjectiveCriteria::read_from(buf)?,
                number_format: NumberFormat::read_from(buf)?,
            }),
        }
    }
}

impl McBufWritable for Method {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match self {
            Method::Add {
                display_name,
                render_type,
                number_format,
            } => {
                MethodKind::Add.write_into(buf)?;
                display_name.write_into(buf)?;
                render_type.write_into(buf)?;
                number_format.write_into(buf)?;
            }
            Method::Remove => MethodKind::Remove.write_into(buf)?,
            Method::Change {
                display_name,
                render_type,
                number_format,
            } => {
                MethodKind::Change.write_into(buf)?;
                display_name.write_into(buf)?;
                render_type.write_into(buf)?;
                number_format.write_into(buf)?;
            }
        }
        Ok(())
    }
}
