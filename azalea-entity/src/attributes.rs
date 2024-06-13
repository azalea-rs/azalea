//! See <https://minecraft.fandom.com/wiki/Attribute>.

use std::collections::HashMap;

use azalea_buf::McBuf;
use azalea_core::resource_location::ResourceLocation;
use bevy_ecs::component::Component;
use thiserror::Error;

#[derive(Clone, Debug, Component)]
pub struct Attributes {
    pub speed: AttributeInstance,
    pub attack_speed: AttributeInstance,
}

#[derive(Clone, Debug)]
pub struct AttributeInstance {
    pub base: f64,
    modifiers_by_id: HashMap<ResourceLocation, AttributeModifier>,
}

#[derive(Clone, Debug, Error)]
#[error("A modifier with this UUID is already present.")]
pub struct AlreadyPresentError;

impl AttributeInstance {
    pub fn new(base: f64) -> Self {
        Self {
            base,
            modifiers_by_id: HashMap::new(),
        }
    }

    pub fn calculate(&self) -> f64 {
        let mut total = self.base;
        for modifier in self.modifiers_by_id.values() {
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
            .modifiers_by_id
            .insert(modifier.id.clone(), modifier)
            .is_some()
        {
            Err(AlreadyPresentError)
        } else {
            Ok(())
        }
    }

    /// Remove the modifier with the given ID from this attribute, returning
    /// the previous modifier is present.
    pub fn remove(&mut self, id: &ResourceLocation) -> Option<AttributeModifier> {
        self.modifiers_by_id.remove(id)
    }
}

#[derive(Clone, Debug, McBuf)]
pub struct AttributeModifier {
    pub id: ResourceLocation,
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
        id: ResourceLocation::new("sprinting"),
        amount: 0.30000001192092896,
        operation: AttributeModifierOperation::MultiplyTotal,
    }
}

pub fn base_attack_speed_modifier(amount: f64) -> AttributeModifier {
    AttributeModifier {
        id: ResourceLocation::new("base_attack_speed"),
        amount,
        operation: AttributeModifierOperation::Addition,
    }
}
