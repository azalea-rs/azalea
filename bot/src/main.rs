use azalea_client::{Account, Client, Event, MoveDirection};
use std::convert::TryInto;

#[tokio::main]
async fn main() {
    env_logger::init();

    let bot = Account::offline("bot");

    let (bot, mut rx) = bot.join(&"localhost".try_into().unwrap()).await.unwrap();

    while let Some(event) = rx.recv().await {
        tokio::spawn(handle_event(event, bot.clone()));
    }
}

async fn handle_event(event: Event, mut bot: Client) -> anyhow::Result<()> {
    match event {
        Event::Login => {
            // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            // bot.walk(MoveDirection::Forward);

            // loop {
            //     tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            // }
            // bot.walk(MoveDirection::None);
        }
        Event::GameTick => {
            bot.set_jumping(true);
        }
        Event::Packet(_packet) => {}
        _ => {}
    }

    Ok(())
}
