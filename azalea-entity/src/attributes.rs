//! Attributes and modifiers for entities.
//!
//! Also see <https://minecraft.wiki/w/Attribute>.

use std::collections::{HashMap, hash_map};

use azalea_core::attribute_modifier_operation::AttributeModifierOperation;
use azalea_inventory::components::AttributeModifier;
use azalea_registry::{builtin::Attribute, identifier::Identifier};
use bevy_ecs::component::Component;
use thiserror::Error;

/// A component that contains the current attribute values for an entity.
///
/// Each attribute can have multiple modifiers, and these modifiers are the
/// result of things like sprinting or enchantments.
#[derive(Clone, Component, Debug)]
pub struct Attributes {
    pub movement_speed: AttributeInstance,
    pub sneaking_speed: AttributeInstance,
    pub attack_speed: AttributeInstance,
    pub water_movement_efficiency: AttributeInstance,
    pub mining_efficiency: AttributeInstance,

    pub block_interaction_range: AttributeInstance,
    pub entity_interaction_range: AttributeInstance,

    pub step_height: AttributeInstance,
}

impl Attributes {
    /// Returns a mutable reference to the [`AttributeInstance`] for the given
    /// attribute, or `None` if the attribute isn't implemented.
    pub fn get_mut(&mut self, attribute: Attribute) -> Option<&mut AttributeInstance> {
        let value = match attribute {
            Attribute::MovementSpeed => &mut self.movement_speed,
            Attribute::SneakingSpeed => &mut self.sneaking_speed,
            Attribute::AttackSpeed => &mut self.attack_speed,
            Attribute::WaterMovementEfficiency => &mut self.water_movement_efficiency,
            Attribute::MiningEfficiency => &mut self.mining_efficiency,
            Attribute::BlockInteractionRange => &mut self.block_interaction_range,
            Attribute::EntityInteractionRange => &mut self.entity_interaction_range,
            Attribute::StepHeight => &mut self.step_height,
            _ => return None,
        };
        Some(value)
    }
}

/// An individual attribute for an entity, which may have any number of
/// modifiers attached to it.
#[derive(Clone, Debug)]
pub struct AttributeInstance {
    pub base: f64,
    modifiers_by_id: HashMap<Identifier, AttributeModifier>,
    // TODO: add cache
}

/// An error for when we try to call [`AttributeInstance::try_insert`] when the
/// modifier is already present.
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
    pub fn remove(&mut self, id: &Identifier) -> Option<AttributeModifier> {
        self.modifiers_by_id.remove(id)
    }
}

pub fn sprinting_modifier() -> AttributeModifier {
    AttributeModifier {
        id: Identifier::new("sprinting"),
        amount: 0.3f32 as f64,
        operation: AttributeModifierOperation::AddMultipliedTotal,
    }
}
pub fn base_attack_speed_modifier(amount: f64) -> AttributeModifier {
    AttributeModifier {
        id: Identifier::new("base_attack_speed"),
        amount,
        operation: AttributeModifierOperation::AddValue,
    }
}
pub fn creative_block_interaction_range_modifier() -> AttributeModifier {
    AttributeModifier {
        id: Identifier::new("creative_mode_block_range"),
        amount: 0.5,
        operation: AttributeModifierOperation::AddValue,
    }
}

pub fn creative_entity_interaction_range_modifier() -> AttributeModifier {
    AttributeModifier {
        id: Identifier::new("creative_mode_entity_range"),
        amount: 2.0,
        operation: AttributeModifierOperation::AddValue,
    }
}
