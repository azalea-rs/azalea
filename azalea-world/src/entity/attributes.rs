//! See <https://minecraft.fandom.com/wiki/Attribute>.

use std::{
    collections::HashMap,
    io::{Cursor, Write},
};

use azalea_buf::{BufReadError, McBuf, McBufReadable, McBufWritable};
use azalea_ecs::component::Component;
use thiserror::Error;
use uuid::{uuid, Uuid};

#[derive(Clone, Debug, Component)]
pub struct Attributes {
    pub speed: AttributeInstance,
}

#[derive(Clone, Debug)]
pub struct AttributeInstance {
    pub base: f64,
    modifiers_by_uuid: HashMap<Uuid, AttributeModifier>,
}

#[derive(Clone, Debug, Error)]
#[error("A modifier with this UUID is already present.")]
pub struct AlreadyPresentError;

impl AttributeInstance {
    pub fn new(base: f64) -> Self {
        Self {
            base,
            modifiers_by_uuid: HashMap::new(),
        }
    }

    pub fn calculate(&self) -> f64 {
        let mut total = self.base;
        for modifier in self.modifiers_by_uuid.values() {
            match modifier.operation {
                AttributeModifierOperation::Addition => total += modifier.amount,
                AttributeModifierOperation::MultiplyBase => total += self.base * modifier.amount,
                _ => {}
            }
            if let AttributeModifierOperation::MultiplyTotal = modifier.operation {
                total *= 1.0 + modifier.amount;
            }
        }
        total
    }

    /// Add a new modifier to this attribute.
    pub fn insert(&mut self, modifier: AttributeModifier) -> Result<(), AlreadyPresentError> {
        if self
            .modifiers_by_uuid
            .insert(modifier.uuid, modifier)
            .is_some()
        {
            Err(AlreadyPresentError)
        } else {
            Ok(())
        }
    }

    /// Remove the modifier with the given UUID from this attribute, returning
    /// the previous modifier is present.
    pub fn remove(&mut self, uuid: &Uuid) -> Option<AttributeModifier> {
        self.modifiers_by_uuid.remove(uuid)
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
