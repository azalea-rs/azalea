#![allow(unused_variables, unused_imports)]
use azalea_client::{Account, Event};
use azalea_core::{PositionXYZ, Vec3};
use azalea_physics::collision::{HasCollision, MoverType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    // let address = "95.111.249.143:10000";
    let address = "localhost";
    // let response = azalea_client::ping::ping_server(&address.try_into().unwrap())
    //     .await
    //     .unwrap();

    // println!("{}", response.description.to_ansi(None));
    let account = Account::offline("bot");
    let (client, mut rx) = account.join(&address.try_into().unwrap()).await.unwrap();
    println!("connected");

    while let Some(e) = &rx.recv().await {
        match e {
            // TODO: have a "loaded" or "ready" event that fires when all chunks are loaded
            Event::Login => {}
            // Event::GameTick => {
            //     let world = client.world();
            //     if let Some(b) = world.find_one_entity(|e| {
            //         e.uuid == uuid::uuid!("6536bfed-8695-48fd-83a1-ecd24cf2a0fd")
            //     }) {
            //         // let world = state.world.as_ref().unwrap();
            //         // world.
            //         println!("{:?}", b);
            //     }
            //     // world.get_block_state(state.player.entity.pos);
            //     // println!("{}", p.message.to_ansi(None));
            //     // if p.message.to_ansi(None) == "<py5> ok" {
            //     //     let state = client.state.lock();
            //     //     let world = state.world.as_ref().unwrap();
            //     //     let c = world.get_block_state(&BlockPos::new(5, 78, -2)).unwrap();
            //     //     println!("block state: {:?}", c);
            //     // }
            // }
            Event::Chat(m) => {
                //     let new_pos = {
                //         let dimension_lock = client.dimension.lock().unwrap();
                //         let player = client.player.lock().unwrap();
                //         let entity = player
                //             .entity(&dimension_lock)
                //             .expect("Player entity is not in world");
                //         entity.pos().add_y(-0.5)
                //     };

                // println!("{:?}", new_pos);
                // client.set_pos(new_pos).await.unwrap();
                // client.move_entity()

                // println!("{}", m.to_ansi(None));
                // if let Err(e) = client
                //     .move_entity(&Vec3 {
                //         x: 0.,
                //         y: -0.5,
                //         z: 0.,
                //     })
                //     .await
                // {
                //     eprintln!("{:?}", e);
                // }
            }
            _ => {}
        }
    }

    println!("done");

    Ok(())
}
