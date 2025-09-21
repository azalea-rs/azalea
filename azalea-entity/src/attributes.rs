//! See <https://minecraft.wiki/w/Attribute>.

use std::collections::{HashMap, hash_map};

use azalea_buf::AzBuf;
use azalea_core::resource_location::ResourceLocation;
use bevy_ecs::component::Component;
use thiserror::Error;

#[derive(Clone, Debug, Component)]
pub struct Attributes {
    pub movement_speed: AttributeInstance,
    pub sneaking_speed: AttributeInstance,
    pub attack_speed: AttributeInstance,
    pub water_movement_efficiency: AttributeInstance,

    pub block_interaction_range: AttributeInstance,
    pub entity_interaction_range: AttributeInstance,

    pub step_height: AttributeInstance,
}

#[derive(Clone, Debug)]
pub struct AttributeInstance {
    pub base: f64,
    modifiers_by_id: HashMap<ResourceLocation, AttributeModifier>,
    // TODO: add cache
}

#[derive(Clone, Debug, Error)]
#[error("A modifier with this ID is already present.")]
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
                AttributeModifierOperation::AddValue => total += modifier.amount,
                AttributeModifierOperation::AddMultipliedBase => {
                    total += modifier.amount * self.base
                }
                AttributeModifierOperation::AddMultipliedTotal => total *= 1. + modifier.amount,
            };
        }
        total
    }

    /// Add a new modifier to this attribute and return the previous value, if
    /// present.
    pub fn insert(&mut self, modifier: AttributeModifier) -> Option<AttributeModifier> {
        self.modifiers_by_id.insert(modifier.id.clone(), modifier)
    }

    /// Insert the given modifier if it's not already present, otherwise returns
    /// [`AlreadyPresentError`].
    pub fn try_insert(&mut self, modifier: AttributeModifier) -> Result<(), AlreadyPresentError> {
        match self.modifiers_by_id.entry(modifier.id.clone()) {
            hash_map::Entry::Occupied(_) => Err(AlreadyPresentError),
            hash_map::Entry::Vacant(entry) => {
                entry.insert(modifier);
                Ok(())
            }
        }
    }

    /// Remove the modifier with the given ID from this attribute, returning
    /// the previous modifier is present.
    pub fn remove(&mut self, id: &ResourceLocation) -> Option<AttributeModifier> {
        self.modifiers_by_id.remove(id)
    }
}

#[derive(Clone, Debug, AzBuf, PartialEq)]
pub struct AttributeModifier {
    pub id: ResourceLocation,
    pub amount: f64,
    pub operation: AttributeModifierOperation,
}

#[derive(Clone, Debug, Copy, AzBuf, PartialEq)]
pub enum AttributeModifierOperation {
    AddValue,
    AddMultipliedBase,
    AddMultipliedTotal,
}

pub fn sprinting_modifier() -> AttributeModifier {
    AttributeModifier {
        id: ResourceLocation::new("sprinting"),
        amount: 0.3f32 as f64,
        operation: AttributeModifierOperation::AddMultipliedTotal,
    }
}
pub fn base_attack_speed_modifier(amount: f64) -> AttributeModifier {
    AttributeModifier {
        id: ResourceLocation::new("base_attack_speed"),
        amount,
        operation: AttributeModifierOperation::AddValue,
    }
}
pub fn creative_block_interaction_range_modifier() -> AttributeModifier {
    AttributeModifier {
        id: ResourceLocation::new("creative_mode_block_range"),
        amount: 0.5,
        operation: AttributeModifierOperation::AddValue,
    }
}

pub fn creative_entity_interaction_range_modifier() -> AttributeModifier {
    AttributeModifier {
        id: ResourceLocation::new("creative_mode_entity_range"),
        amount: 2.0,
        operation: AttributeModifierOperation::AddValue,
    }
}
