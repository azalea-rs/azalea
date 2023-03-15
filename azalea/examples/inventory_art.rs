//! Take the items in a container and put one of each in a checkerboard pattern

use azalea::{pathfinder::BlockPosGoal, prelude::*};
use parking_lot::Mutex;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let account = Account::offline("bot");
    // or let bot = Account::microsoft("email").await;

    ClientBuilder::new()
        .set_handler(handle)
        .start(account, "localhost")
        .await
        .unwrap();
}

#[derive(Default, Clone, Component)]
struct State {
    pub started: Arc<Mutex<bool>>,
}

async fn handle(mut bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    match event {
        Event::Chat(m) => {
            if m.username() == Some(bot.profile.name.clone()) {
                return Ok(());
            };
            if m.content() != "go" {
                return Ok(());
            }
            {
                // make sure we only start once
                if *state.started.lock() {
                    return Ok(());
                };
                *state.started.lock() = true;
            }

            let chest_block = bot
                .world()
                .read()
                .find_block(bot.position(), &azalea::Block::Chest.into());
            let Some(chest_block) = chest_block else {
                bot.chat("No chest found");
                return Ok(());
            };
            bot.goto(BlockPosGoal::from(chest_block));
            let Some(chest) = bot.open_container(chest_block).await else {
                println!("Couldn't open chest");
                return Ok(());
            };

            // move everything into our inventory first
        }
        _ => {}
    }

    Ok(())
}
