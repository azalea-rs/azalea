use std::sync::Arc;

use azalea::{pathfinder, Account, Accounts, Client, Event};
use parking_lot::Mutex;

#[tokio::main]
async fn main() {
    let accounts = Accounts::new();

    for i in 0..10 {
        accounts.add(Account::offline(format!("bot{}", i)));
    }

    azalea::start_swarm(azalea::SwarmOptions {
        accounts,
        address: "localhost",

        swarm_state: Arc::new(Mutex::new(State::default())),
        state: State::default(),

        swarm_plugins: vec![Arc::new(pathfinder::Plugin::default())],
        plugins: vec![],

        handle: Box::new(handle),
        swarm_handle: Box::new(handle),
    })
    .await
    .unwrap();
}

struct State {}
struct SwarmState {}

async fn handle(bots: Client, event: Arc<Event>, state: Arc<Mutex<State>>) {
    match *event {
        Event::Tick => {
            // choose an arbitrary player within render distance to target
            if let Some(target) = bots
                .dimension()
                .find_one_entity(|e| e.id == "minecraft:player")
            {
                for bot in bots {
                    bot.tick_goto_goal(pathfinder::Goals::Reach(target.bounding_box));
                    // if target.bounding_box.distance(bot.eyes) < bot.reach_distance() {
                    if bot.entity.can_reach(target.bounding_box) {
                        bot.swing();
                    }
                    if !h.using_held_item() && bot.state.lock().hunger <= 17 {
                        bot.hold(azalea::ItemGroup::Food);
                        tokio::task::spawn(bot.use_held_item());
                    }
                }
            }
        }
        _ => {}
    }
}
