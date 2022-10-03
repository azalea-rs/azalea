use azalea::prelude::*;
use azalea::{Account, Client, Event};
use parking_lot::Mutex;
use std::sync::Arc;

#[derive(Default)]
struct State {}

#[tokio::main]
async fn main() {
    env_logger::init();

    let account = Account::offline("bot");

    azalea::start(azalea::Options {
        account,
        address: "localhost",
        state: Arc::new(Mutex::new(State::default())),
        plugins: vec![],
        handle: Box::new(handle),
    })
    .await
    .unwrap();
}

async fn handle(bot: Client, event: Arc<Event>, _state: Arc<Mutex<State>>) -> anyhow::Result<()> {
    match *event {
        Event::GameTick => {
            bot.jump();
        }
        _ => {}
    }

    Ok(())
}
