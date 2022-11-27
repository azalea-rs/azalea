use azalea::{pathfinder, Account, Client, Event, SwarmEvent};
use azalea::{prelude::*, Swarm};

#[tokio::main]
async fn main() {
    let mut accounts = Vec::new();
    let mut states = Vec::new();

    for i in 0..10 {
        accounts.push(Account::offline(&format!("bot{}", i)));
        states.push(State::default());
    }

    azalea::start_swarm(azalea::SwarmOptions {
        accounts,
        address: "localhost",

        swarm_state: SwarmState::default(),
        states,

        swarm_plugins: swarm_plugins![pathfinder::Plugin],
        plugins: plugins![],

        handle,
        swarm_handle,

        join_delay: None,
    })
    .await
    .unwrap();
}

#[derive(Default, Clone)]
struct State {}

#[derive(Default, Clone)]
struct SwarmState {}

async fn handle(bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    Ok(())
}
async fn swarm_handle(
    swarm: Swarm<State>,
    event: SwarmEvent,
    state: SwarmState,
) -> anyhow::Result<()> {
    match event {
        SwarmEvent::Tick => {
            // choose an arbitrary player within render distance to target
            if let Some(target) = swarm
                .worlds
                .read()
                .find_one_entity(|e| e.id == "minecraft:player")
            {
                for (bot, bot_state) in swarm {
                    bot.tick_goto_goal(pathfinder::Goals::Reach(target.bounding_box));
                    // if target.bounding_box.distance(bot.eyes) < bot.reach_distance() {
                    if bot.entity().can_reach(target.bounding_box) {
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
