use azalea::{pathfinder, Account, Client, Event};

#[tokio::main]
async fn main() {
    let accounts = Vec::new();

    for i in 0..10 {
        accounts.push(Account::offline(&format!("bot{}", i)));
    }

    azalea::start_swarm(azalea::SwarmOptions {
        accounts,
        address: "localhost",

        swarm_state: State::default(),
        state: State::default(),

        swarm_plugins: vec![Box::new(pathfinder::Plugin::default())],
        plugins: vec![],

        handle: Box::new(handle),
        swarm_handle: Box::new(swarm_handle),
    })
    .await
    .unwrap();
}

#[derive(Default, Clone)]
struct State {}

#[derive(Default, Clone)]
struct SwarmState {}

async fn handle(bot: Client, event: Event, state: State) {}
async fn swarm_handle(swarm: Swarm, event: Event, state: State) {
    match event {
        Event::Tick => {
            // choose an arbitrary player within render distance to target
            if let Some(target) = swarm
                .dimension
                .find_one_entity(|e| e.id == "minecraft:player")
            {
                for bot in swarm {
                    bot.tick_goto_goal(pathfinder::Goals::Reach(target.bounding_box));
                    // if target.bounding_box.distance(bot.eyes) < bot.reach_distance() {
                    if bot.entity.can_reach(target.bounding_box) {
                        bot.swing();
                    }
                    if !bot.using_held_item() && bot.state.lock().hunger <= 17 {
                        bot.hold(azalea::ItemGroup::Food);
                        tokio::task::spawn(bot.use_held_item());
                    }
                }
            }
        }
        _ => {}
    }
}
