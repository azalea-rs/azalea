use azalea::{Account, Accounts, Event, pathfinder};

#[tokio::main]
async fn main() {
    let accounts = Accounts::new();
    for i in 0..10 {
        accounts.add(Account::offline(format!("bot{}", i)));
    }

    let bots = accounts.join("localhost".try_into().unwrap()).await.unwrap();

    match bots.next().await {
        Event::Tick {
            // choose an arbitrary player within render distance to target
            if let Some(target) = bots.world.find_one_entity(|e| e.id == "minecraft:player") {
                for bot in bots {
                    bot.tick_goto_goal(
                        pathfinder::Goals::Reach(target.bounding_box)
                    );
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
        },
        _ => {}
    }
}
