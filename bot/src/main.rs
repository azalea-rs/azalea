use azalea::prelude::*;
use azalea::{Account, Client, Event};
use parking_lot::Mutex;
use std::sync::Arc;

#[derive(Default)]
struct State {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let account = Account::microsoft("example2@example.com").await?;

    azalea::start(azalea::Options {
        account,
        address: "localhost",
        state: Arc::new(Mutex::new(State::default())),
        plugins: vec![],
        handle,
    })
    .await
    .unwrap();

    Ok(())
}

async fn handle(bot: Client, event: Arc<Event>, _state: Arc<Mutex<State>>) -> anyhow::Result<()> {
    if let Event::Tick = *event {
        bot.jump();
    }

    Ok(())
}
