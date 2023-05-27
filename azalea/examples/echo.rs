//! A simple bot that repeats chat messages sent by other players.

use azalea::prelude::*;

#[tokio::main]
async fn main() {
    // let account = Account::offline("bot");
    let account = Account::microsoft("minecraft3@matdoes.dev").await.unwrap();

    ClientBuilder::new()
        .set_handler(handle)
        .start(account, "85.190.131.233")
        .await
        .unwrap();
}

#[derive(Default, Clone, Component)]
pub struct State {}

async fn handle(bot: Client, event: Event, _state: State) -> anyhow::Result<()> {
    std::process::exit(0);
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
