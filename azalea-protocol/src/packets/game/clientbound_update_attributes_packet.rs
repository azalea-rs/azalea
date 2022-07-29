use azalea_buf::McBuf;
use azalea_buf::{McBufReadable, McBufWritable, Readable, Writable};
use azalea_core::ResourceLocation;
use packet_macros::ClientboundGamePacket;
use std::io::{Read, Write};
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
    fn read_from(buf: &mut impl Read) -> Result<Self, String> {
        match buf.read_byte()? {
            0 => Ok(Operation::Addition),
            1 => Ok(Operation::MultiplyBase),
            2 => Ok(Operation::MultiplyTotal),
            op => Err(format!("Unknown operation: {}", op)),
        }
    }
}

impl McBufWritable for Operation {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        buf.write_byte(*self as u8)?;
        Ok(())
    }
}
