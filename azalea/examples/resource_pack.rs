//! A simple bot that repeats chat messages sent by other players.

use azalea::prelude::*;
use azalea_protocol::packets::game::ClientboundGamePacket;

#[tokio::main]
async fn main() {
    let account = Account::offline("bot");
    // or let account = Account::microsoft("email").await.unwrap();

    ClientBuilder::new()
        .set_handler(handle)
        .reconnect_after(None)
        .start(account, "localhost")
        .await
        .unwrap();
}

#[derive(Default, Clone, Component)]
pub struct State {}

async fn handle(bot: Client, event: Event, _state: State) -> anyhow::Result<()> {
    match event {
        Event::Packet(packet) => {
            if let ClientboundGamePacket::ResourcePackPush(push_pack) = packet.as_ref() {
                println!("Resource Pack URL: {}", push_pack.url);

                bot.disconnect();
            }
        }
        Event::Disconnect(_) => {
            std::process::exit(0);

        }
        _ => {}
    }

    Ok(())
}
