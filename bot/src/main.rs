use azalea_client::{Account, Event};
use azalea_core::{BlockPos, ChunkPos};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // let address = "95.111.249.143:10000";
    let address = "192.168.2.234:62840";
    // let response = azalea_client::ping::ping_server(&address.try_into().unwrap())
    //     .await
    //     .unwrap();

    // println!("{}", response.description.to_ansi(None));
    let account = Account::offline("bot");
    let mut client = account.join(&address.try_into().unwrap()).await.unwrap();
    println!("connected");

    while let Some(e) = client.next().await {
        match e {
            // TODO: have a "loaded" or "ready" event that fires when all chunks are loaded
            Event::Login => {}
            Event::Chat(p) => {
                println!("{}", p.message.to_ansi(None));
                if p.message.to_ansi(None) == "<py5> ok" {
                    let state = client.state.lock().await;
                    let world = state.world.as_ref().unwrap();
                    // let c = world[&BlockPos::new(5, 78, -2)]
                    //     .as_ref()
                    //     .unwrap()
                    //     .lock()
                    //     .unwrap();
                    // println!("{:?}", c);
                }
            }
        }
    }

    println!("done");
}
