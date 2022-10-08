use std::sync::Arc;

use azalea::{Account, Client, Event};
use parking_lot::Mutex;

#[tokio::main]
async fn main() {
    let account = Account::offline("bot");
    // or let account = azalea::Account::microsoft("access token").await;

    azalea::start(azalea::Options {
        account,
        address: "localhost",
        state: Arc::new(Mutex::new(State::default())),
        plugins: vec![],
        handle,
    })
    .await
    .unwrap();
}

pub struct State {}

async fn handle(bot: Client, event: Arc<Event>, state: Arc<Mutex<State>>) -> anyhow::Result<()> {
    match event {
        Event::Chat(m) => {
            if m.username == bot.username {
                return Ok(()); // ignore our own messages
            };
            bot.chat(m.message).await;
        }
        Event::Kick(m) => {
            println!(m);
            bot.reconnect().await.unwrap();
        }
        Event::HungerUpdate(h) => {
            if !h.using_held_item() && h.hunger <= 17 {
                bot.hold(azalea::ItemGroup::Food).await?;
                bot.use_held_item().await?;
            }
        }
        _ => {}
    }

    Ok(())
}
