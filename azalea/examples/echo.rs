//! A simple bot that repeats chat messages sent by other players.

use azalea::{Account, Client, Event};
use parking_lot::Mutex;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let account = Account::offline("bot");
    // or let account = Account::microsoft("email").await;

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
    match *event {
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
        _ => {}
    }

    Ok(())
}
