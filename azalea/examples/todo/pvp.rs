use std::time::Duration;

use azalea::{
    Account, Client, Event, GameProfileComponent, ecs::query::With, entity::metadata::Player,
    pathfinder, prelude::*, swarm::prelude::*,
};

#[tokio::main]
async fn main() -> AppExit {
    let mut accounts = Vec::new();
    let mut states = Vec::new();

    for i in 0..10 {
        accounts.push(Account::offline(&format!("bot{i}")));
        states.push(State::default());
    }

    SwarmBuilder::new()
        .add_accounts(accounts.clone())
        .set_handler(handle)
        .set_swarm_handler(swarm_handle)
        .join_delay(Duration::from_millis(1000))
        .start("localhost")
        .await
}

#[derive(Clone, Component, Default)]
struct State {}

#[derive(Clone, Default, Resource)]
struct SwarmState {}

async fn handle(bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    Ok(())
}
async fn swarm_handle(swarm: Swarm, event: SwarmEvent, state: SwarmState) -> anyhow::Result<()> {
    match event {
        SwarmEvent::Tick => {
            if let Some(target_entity) =
                swarm.entity_by::<With<Player>>(|profile: &&GameProfileComponent| {
                    profile.name == "Herobrine"
                })
            {
                let target_bounding_box =
                    swarm.map_entity(target_entity, |bb: &BoundingBox| bb.clone());

                for (bot, bot_state) in swarm {
                    bot.tick_goto_goal(pathfinder::Goals::Reach(target_bounding_box));
                    // if target.bounding_box.distance(bot.eyes) < bot.reach_distance() {
                    if bot.can_reach(target_bounding_box) {
                        bot.swing();
                    }
                    if !bot.using_held_item() && bot.hunger() <= 17 {
                        bot.hold(azalea::ItemGroup::Food);
                        tokio::task::spawn(bot.use_held_item());
                    }
                }
            }
        }
        _ => {}
    }

    Ok(())
}
