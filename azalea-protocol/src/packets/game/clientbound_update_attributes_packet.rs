use crate::mc_buf::{McBufReadable, McBufWritable, Readable, Writable};
use azalea_core::resource_location::ResourceLocation;
use packet_macros::{GamePacket, McBufReadable, McBufWritable};
use std::io::{Read, Write};
use uuid::Uuid;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundUpdateAttributesPacket {
    #[varint]
    pub entity_id: u32,
    pub attributes: Vec<AttributeSnapshot>,
}

#[derive(Clone, Debug, McBufReadable, McBufWritable)]
pub struct AttributeSnapshot {
    pub attribute: ResourceLocation,
    pub base: f64,
    pub modifiers: Vec<Modifier>,
}

#[derive(Clone, Debug, McBufReadable, McBufWritable)]
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
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
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
