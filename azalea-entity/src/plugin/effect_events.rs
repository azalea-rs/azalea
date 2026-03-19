use azalea_registry::builtin::MobEffect;
use bevy_ecs::{entity::Entity, event::EntityEvent, observer::On, system::Query};
use tracing::warn;

use crate::{ActiveEffects, Attributes, MobEffectData, effects::attribute_modifier_for_effect};

#[derive(EntityEvent)]
pub struct AddEffectEvent {
    pub entity: Entity,
    pub id: MobEffect,
    pub data: MobEffectData,
}
pub fn handle_add_effect(
    add_effect: On<AddEffectEvent>,
    mut query: Query<(&mut ActiveEffects, &mut Attributes)>,
) {
    let Ok((mut active_effects, mut attributes)) = query.get_mut(add_effect.entity) else {
        warn!("got handle_add_effect for an entity without the required components");
        return;
    };

    active_effects.insert(add_effect.id, add_effect.data.clone());

    if let Some((attribute, modifier)) = attribute_modifier_for_effect(add_effect.id) {
        let modifier = modifier.create(add_effect.data.amplifier);
        if let Some(attribute) = attributes.get_mut(attribute) {
            attribute.insert(modifier);
        }
    }
}

#[derive(EntityEvent)]
pub struct RemoveEffectsEvent {
    pub entity: Entity,
    pub effects: Vec<MobEffect>,
}
pub fn handle_remove_effects(
    remove_effects: On<RemoveEffectsEvent>,
    mut query: Query<(&mut ActiveEffects, &mut Attributes)>,
) {
    let Ok((mut active_effects, mut attributes)) = query.get_mut(remove_effects.entity) else {
        warn!("got handle_remove_effects for an entity without the required components");
        return;
    };

    for &effect in &remove_effects.effects {
        active_effects.remove(effect);

        if let Some((attribute, modifier)) = attribute_modifier_for_effect(effect) {
            // we're just trying to get the id of the modifier, so the amplifier passed here
            // doesn't matter
            let modifier = modifier.create(0);
            if let Some(attribute) = attributes.get_mut(attribute) {
                attribute.remove(&modifier.id);
            }
        }
    }
}
