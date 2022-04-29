# Azalea

A Rust library for creating Minecraft bots.

I named this Azalea because it sounds like a cool word and this is a cool library. This project was heavily inspired by PrismarineJS.

## Goals

- Do everything a vanilla client can do
- Be easy to use
- Bypass most/all anticheats
- Support the latest Minecraft version
- Be fast

## Example code

Note that this doesn't work yet, it's just how I want the API to look.

```rs
use azalea::{Account, Event};

let account = Account::offline("bot");
// or let account = azalea::Account::microsoft("access token").await;

let bot = account.join("localhost".try_into().unwrap()).await.unwrap();

loop {
    match bot.next().await {
        Event::Message(m) {
            if m.username == bot.username { return };
            bot.chat(m.message).await;
        },
        Event::Kicked(m) {
            println!(m);
            bot.reconnect().await.unwrap();
        },
        _ => {}
    }
}
```

You can use the `azalea::Bots` struct to control many bots as one unit.

```rs
use azalea::{Account, Accounts, Event, pathfinder};

#[tokio::main]
async fn main() {
    let accounts = Accounts::new();

    for i in 0..10 {
        accounts.add(Account::offline(format!("bot{}", i)));
    }

    let bots = accounts.join("localhost".try_into().unwrap()).await.unwrap();

    bots.goto(pathfinder::GotoGoal(azalea::BlockCoord(0, 70, 0))).await;

    // destroy the blocks in this area and then leave

    bots.fill(
        pathfinder::FillGoal(
            azalea::BlockCoord(-5, 60, -5),
            azalea::BlockCoord(5, 70, 5)
        ),
        azalea::block::Air
    ).await;
}
```
