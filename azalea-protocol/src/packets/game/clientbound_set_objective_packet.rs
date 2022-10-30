use azalea_buf::{BufReadError, McBuf, McBufReadable, McBufWritable};
use azalea_chat::Component;
use azalea_protocol_macros::ClientboundGamePacket;
use std::io::{Cursor, Write};

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSetObjectivePacket {
    pub objective_name: String,
    pub method: Method,
}

#[derive(Clone, Debug)]
pub enum Method {
    Add(DisplayInfo),
    Remove,
    Change(DisplayInfo),
}

impl McBufReadable for Method {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(match u8::read_from(buf)? {
            0 => Method::Add(DisplayInfo::read_from(buf)?),
            1 => Method::Remove,
            2 => Method::Change(DisplayInfo::read_from(buf)?),
            id => return Err(BufReadError::UnexpectedEnumVariant { id: id as i32 }),
        })
    }
}

impl McBufWritable for Method {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match self {
            Method::Add(info) => {
                0u8.write_into(buf)?;
                info.write_into(buf)?;
            }
            Method::Remove => {
                1u8.write_into(buf)?;
            }
            Method::Change(info) => {
                2u8.write_into(buf)?;
                info.write_into(buf)?;
            }
        }
        Ok(())
    }
}

#[derive(McBuf, Clone, Debug)]
pub struct DisplayInfo {
    pub display_name: Component,
    pub render_type: RenderType,
}

#[derive(McBuf, Copy, Clone, Debug)]
pub enum RenderType {
    Integer,
    Hearts,
}
