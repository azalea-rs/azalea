//! https://minecraft.fandom.com/wiki/Attribute

use std::io::{Cursor, Write};

use azalea_buf::{BufReadError, McBuf, McBufReadable, McBufWritable};
use uuid::{uuid, Uuid};

#[derive(Clone, Debug, Default)]
pub struct AttributeModifiers {
    pub speed: AttributeInstance,
}

#[derive(Clone, Debug, Default)]
pub struct AttributeInstance {
    pub base: f64,
    pub modifiers: Vec<AttributeModifier>,
}

impl AttributeInstance {
    pub fn calculate(&self) -> f64 {
        let mut total = self.base;
        for modifier in self.modifiers {
            match modifier.operation {
                AttributeModifierOperation::Addition => total += modifier.amount,
                AttributeModifierOperation::MultiplyBase => total += self.base * modifier.amount,
                _ => {}
            }
            match modifier.operation {
                AttributeModifierOperation::MultiplyTotal => total *= 1.0 + modifier.amount,
                _ => {}
            }
        }
        total
    }
}

#[derive(Clone, Debug)]
pub struct AttributeModifier {
    pub uuid: Uuid,
    pub name: String,
    pub amount: f64,
    pub operation: AttributeModifierOperation,
}

#[derive(Clone, Debug, Copy, McBuf)]
pub enum AttributeModifierOperation {
    Addition,
    MultiplyBase,
    MultiplyTotal,
}

pub fn sprinting_modifier() -> AttributeModifier {
    AttributeModifier {
        uuid: uuid!("662A6B8D-DA3E-4C1C-8813-96EA6097278D"),
        name: "Sprinting speed boost".to_string(),
        amount: 0.30000001192092896,
        operation: AttributeModifierOperation::MultiplyTotal,
    }
}

impl McBufReadable for AttributeModifier {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let uuid = Uuid::read_from(buf)?;
        let amount = f64::read_from(buf)?;
        let operation = AttributeModifierOperation::read_from(buf)?;
        Ok(Self {
            uuid,
            name: "Unknown synced attribute modifier".to_string(),
            amount,
            operation,
        })
    }
}

impl McBufWritable for AttributeModifier {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        self.uuid.write_into(buf)?;
        self.amount.write_into(buf)?;
        self.operation.write_into(buf)?;
        Ok(())
    }
}
