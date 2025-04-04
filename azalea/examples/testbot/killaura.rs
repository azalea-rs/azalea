use azalea::{
    ecs::prelude::*,
    entity::{Dead, LocalEntity, Position, metadata::AbstractMonster},
    prelude::*,
    world::{InstanceName, MinecraftEntityId},
};

use crate::State;

pub fn tick(bot: Client, state: State) -> anyhow::Result<()> {
    if !state.killaura {
        return Ok(());
    }
    if bot.has_attack_cooldown() {
        return Ok(());
    }
    let mut nearest_entity = None;
    let mut nearest_distance = f64::INFINITY;
    let bot_position = bot.eye_position();
    let bot_instance_name = bot.component::<InstanceName>();
    {
        let mut ecs = bot.ecs.lock();
        let mut query = ecs
            .query_filtered::<(&MinecraftEntityId, &Position, &InstanceName), (
                With<AbstractMonster>,
                Without<LocalEntity>,
                Without<Dead>,
            )>();
        for (&entity_id, position, instance_name) in query.iter(&ecs) {
            if instance_name != &bot_instance_name {
                continue;
            }

            let distance = bot_position.distance_to(position);
            if distance < 4. && distance < nearest_distance {
                nearest_entity = Some(entity_id);
                nearest_distance = distance;
            }
        }
    }
    if let Some(nearest_entity) = nearest_entity {
        println!("attacking {:?}", nearest_entity);
        println!("distance {:?}", nearest_distance);
        bot.attack(nearest_entity);
    }

    Ok(())
}
