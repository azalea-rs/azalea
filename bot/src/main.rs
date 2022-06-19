use azalea_client::{Account, Event};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    // let address = "95.111.249.143:10000";
    let address = "localhost:52909";
    // let response = azalea_client::ping::ping_server(&address.try_into().unwrap())
    //     .await
    //     .unwrap();

    // println!("{}", response.description.to_ansi(None));
    let account = Account::offline("bot");
    let mut client = account.join(&address.try_into().unwrap()).await.unwrap();
    println!("connected");

    while let Some(e) = &client.next().await {
        match e {
            // TODO: have a "loaded" or "ready" event that fires when all chunks are loaded
            Event::Login => {}
            Event::Chat(_p) => {
                let state = client.state.lock().unwrap();
                let world = state.world.as_ref().unwrap();
                println!("{:?}", world);
                // world.get_block_state(state.player.entity.pos);
                // println!("{}", p.message.to_ansi(None));
                // if p.message.to_ansi(None) == "<py5> ok" {
                //     let state = client.state.lock();
                //     let world = state.world.as_ref().unwrap();
                //     let c = world.get_block_state(&BlockPos::new(5, 78, -2)).unwrap();
                //     println!("block state: {:?}", c);
                // }
            }
        }
    }

    println!("done");

    Ok(())
}
