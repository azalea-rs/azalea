//! A simple bot that repeats chat messages sent by other players.

use azalea::prelude::*;

#[tokio::main]
async fn main() {
    let account = Account::offline("bot");
    // or let account = Account::microsoft("email").await;

    ClientBuilder::new()
        .set_handler(handle)
        .start(account, "localhost")
        .await
        .unwrap();
}

#[derive(Default, Clone, Component)]
pub struct State {}

async fn handle(bot: Client, event: Event, _state: State) -> anyhow::Result<()> {
    match event {
        Event::Chat(m) => {
            if let (Some(sender), content) = m.split_sender_and_content() {
                if sender == bot.profile.name {
                    return Ok(()); // ignore our own messages
                }
                bot.chat(&content);
            };
        }
        _ => {}
    }

    Ok(())
}
