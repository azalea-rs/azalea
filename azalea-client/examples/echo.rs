//! A simple bot that repeats chat messages sent by other players.

use azalea_client::{Account, Client, Event};

#[tokio::main]
async fn main() {
    let account = Account::offline("bot");
    // or let account = Account::microsoft("email").await;

    let (client, mut rx) = Client::join(&account, "localhost").await.unwrap();

    while let Some(event) = rx.recv().await {
        match &event {
            Event::Chat(m) => {
                if let (Some(sender), content) = m.split_sender_and_content() {
                    if sender == client.profile.name {
                        continue; // ignore our own messages
                    }
                    client.chat(&content).await;
                };
            }
            _ => {}
        }
    }
}
