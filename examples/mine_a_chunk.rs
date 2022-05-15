use azalea::{Account, Accounts, Event, pathfinder};

// You can use the `azalea::Bots` struct to control many bots as one unit.

#[tokio::main]
async fn main() {
    let accounts = Accounts::new();

    for i in 0..10 {
        accounts.add(Account::offline(format!("bot{}", i)));
    }

    let bots = accounts.join("localhost".try_into().unwrap()).await.unwrap();

    bots.goto(azalea::BlockCoord(0, 70, 0)).await;
    // or bots.goto_goal(pathfinder::Goals::Goto(azalea::BlockCoord(0, 70, 0))).await;

    // destroy the blocks in this area and then leave

    bots.fill(
        azalea::Selection::Range(
            azalea::BlockCoord(0, 0, 0),
            azalea::BlockCoord(16, 255, 16)
        ),
        azalea::block::Air
    ).await;
}
