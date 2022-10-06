use azalea_buf::{BufReadError, McBuf};
use azalea_buf::{McBufReadable, McBufWritable};
use azalea_core::ResourceLocation;
use azalea_protocol_macros::ClientboundGamePacket;
use std::io::Write;
use uuid::Uuid;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundUpdateAttributesPacket {
    #[var]
    pub entity_id: u32,
    pub attributes: Vec<AttributeSnapshot>,
}

#[derive(Clone, Debug, McBuf)]
pub struct AttributeSnapshot {
    pub attribute: ResourceLocation,
    pub base: f64,
    pub modifiers: Vec<Modifier>,
}

#[derive(Clone, Debug, McBuf)]
pub struct Modifier {
    pub uuid: Uuid,
    pub amount: f64,
    pub operation: u8,
}

#[derive(Clone, Debug, Copy)]
enum Operation {
    Addition = 0,
    MultiplyBase = 1,
    MultiplyTotal = 2,
}

impl McBufReadable for Operation {
    fn read_from(buf: &mut Cursor<Vec<u8>>) -> Result<Self, BufReadError> {
        match u8::read_from(buf)? {
            0 => Ok(Operation::Addition),
            1 => Ok(Operation::MultiplyBase),
            2 => Ok(Operation::MultiplyTotal),
            id => Err(BufReadError::UnexpectedEnumVariant { id: id.into() }),
        }
    }
}

impl McBufWritable for Operation {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        (*self as u8).write_into(buf)?;
        Ok(())
    }
}
