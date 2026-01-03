//! Support for enchantments and items with attribute modifiers.

use std::collections::HashMap;

use azalea_core::{data_registry::ResolvableDataRegistry, registry_holder::value::AttributeEffect};
use azalea_entity::{Attributes, inventory::Inventory};
use azalea_inventory::{
    ItemStack,
    components::{self, AttributeModifier, EquipmentSlot},
};
use azalea_registry::identifier::Identifier;
use bevy_ecs::{
    component::Component,
    entity::Entity,
    event::EntityEvent,
    observer::On,
    query::With,
    system::{Commands, Query},
};
use tracing::{debug, error, warn};

use crate::local_player::InstanceHolder;

/// A component that contains the equipment slots that we had last tick.
///
/// This is used by [`collect_equipment_changes`] for applying enchantments.
#[derive(Component, Debug, Default)]
pub struct LastEquipmentItems {
    pub map: HashMap<EquipmentSlot, ItemStack>,
}

pub fn collect_equipment_changes(
    mut commands: Commands,
    mut query: Query<(Entity, &Inventory, Option<&LastEquipmentItems>), With<Attributes>>,
) {
    for (entity, inventory, last_equipment_items) in &mut query {
        let last_equipment_items = if let Some(e) = last_equipment_items {
            e
        } else {
            commands
                .entity(entity)
                .insert(LastEquipmentItems::default());
            continue;
        };

        let mut changes = HashMap::new();

        for equipment_slot in EquipmentSlot::values() {
            let current_item = inventory
                .get_equipment(equipment_slot)
                .unwrap_or(&ItemStack::Empty);
            let last_item = last_equipment_items
                .map
                .get(&equipment_slot)
                .unwrap_or(&ItemStack::Empty);

            if current_item == last_item {
                // item hasn't changed, nothing to do
                continue;
            }

            changes.insert(
                equipment_slot,
                EquipmentChange {
                    old: last_item.clone(),
                    new: current_item.clone(),
                },
            );
        }

        if changes.is_empty() {
            continue;
        }
        commands.trigger(EquipmentChangesEvent {
            entity,
            map: changes,
        });
    }
}

#[derive(Debug, EntityEvent)]
pub struct EquipmentChangesEvent {
    pub entity: Entity,
    pub map: HashMap<EquipmentSlot, EquipmentChange>,
}
#[derive(Debug)]
pub struct EquipmentChange {
    pub old: ItemStack,
    pub new: ItemStack,
}

pub fn handle_equipment_changes(
    equipment_changes: On<EquipmentChangesEvent>,
    mut query: Query<(&InstanceHolder, &mut LastEquipmentItems, &mut Attributes)>,
) {
    let Ok((instance_holder, mut last_equipment_items, mut attributes)) =
        query.get_mut(equipment_changes.entity)
    else {
        error!(
            "got EquipmentChangesEvent with unknown entity {}",
            equipment_changes.entity
        );
        return;
    };

    if !equipment_changes.map.is_empty() {
        debug!("equipment changes: {:?}", equipment_changes.map);
    }

    for (&slot, change) in &equipment_changes.map {
        if change.old.is_present() {
            // stopLocationBasedEffects

            for (attribute, modifier) in
                collect_attribute_modifiers_from_item(slot, &change.old, instance_holder)
            {
                if let Some(attribute) = attributes.get_mut(attribute) {
                    attribute.remove(&modifier.id);
                }
            }

            last_equipment_items.map.remove(&slot);
        }

        if change.new.is_present() {
            // see ItemStack.forEachModifier in vanilla

            for (attribute, modifier) in
                collect_attribute_modifiers_from_item(slot, &change.new, instance_holder)
            {
                if let Some(attribute) = attributes.get_mut(attribute) {
                    attribute.remove(&modifier.id);
                    attribute.insert(modifier);
                }
            }

            // runLocationChangedEffects

            last_equipment_items.map.insert(slot, change.new.clone());
        }
    }
}

fn collect_attribute_modifiers_from_item(
    slot: EquipmentSlot,
    item: &ItemStack,
    instance_holder: &InstanceHolder,
) -> Vec<(azalea_registry::builtin::Attribute, AttributeModifier)> {
    let mut modifiers = Vec::new();

    // handle the attribute_modifiers component first
    let attribute_modifiers = item
        .get_component::<components::AttributeModifiers>()
        .unwrap_or_default();
    for modifier in &attribute_modifiers.modifiers {
        modifiers.push((modifier.kind, modifier.modifier.clone()));
    }

    // now handle enchants
    let enchants = item
        .get_component::<components::Enchantments>()
        .unwrap_or_default();
    if !enchants.levels.is_empty() {
        let registry_holder = &instance_holder.instance.read().registries;
        for (enchant, &level) in &enchants.levels {
            let Some((_enchant_id, enchant_definition)) = enchant.resolve(registry_holder) else {
                warn!(
                    "Got equipment with an enchantment that wasn't in the registry, so it couldn't be resolved to an ID"
                );
                continue;
            };

            let effects = enchant_definition.get::<AttributeEffect>();
            for effect in effects.unwrap_or_default() {
                // TODO: check if the effect definition allows the slot

                let modifier = AttributeModifier {
                    id: Identifier::new(format!("{}/{slot}", effect.id)),
                    amount: effect.amount.calculate(level) as f64,
                    operation: effect.operation,
                };

                modifiers.push((effect.attribute, modifier));
            }
        }
    }

    modifiers
}
