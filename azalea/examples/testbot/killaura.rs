use azalea::{
    ecs::prelude::*,
    entity::{Dead, LocalEntity, Position, metadata::AbstractMonster},
    prelude::*,
};

use crate::State;

pub fn tick(bot: Client, state: State) -> anyhow::Result<()> {
    if !state.killaura {
        return Ok(());
    }
    if bot.has_attack_cooldown() {
        return Ok(());
    }
    let bot_position = bot.eye_position();

    let nearest_entity = bot.nearest_entity_by::<&Position, (
        With<AbstractMonster>,
        Without<LocalEntity>,
        Without<Dead>,
    )>(|position: &Position| {
        let distance = bot_position.distance_to(**position);
        distance < 4.
    });

    if let Some(nearest_entity) = nearest_entity {
        println!("attacking {nearest_entity:?}");
        nearest_entity.attack();
    }

    Ok(())
}
