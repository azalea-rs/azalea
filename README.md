# Azalea

<p align="center">
    <img src="https://cdn.matdoes.dev/images/flowering_azalea.webp" alt="Azalea" height="200">
</p>

A Rust crate for creating Minecraft bots.

I named this Azalea because it sounds like a cool word and this is a cool library. This project was heavily inspired by PrismarineJS.

## Why

I wanted a fun excuse to do something cool with Rust, and I also felt like I could do better than [Mineflayer](https://github.com/prismarinejs/mineflayer) in some areas.

## Goals

- Do everything a vanilla client can do
- Be easy to use
- Bypass most/all anticheats
- Support the latest Minecraft version
- Be fast and memory efficient

## Example code

Note that these doesn't work yet, it's just how I want the API to look.

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

```rs
use azalea::{Bot, Event};

let bot = Bot::offline("bot");
// or let bot = azalea::Bot::microsoft("access token").await;

bot.join("localhost".try_into().unwrap()).await.unwrap();

loop {
    match bot.recv().await {
        Event::Message(m) {
            if m.username == bot.username { return };
            if m.message = "go" {
                bot.goto_goal(
                    pathfinder::Goals::NearXZ(5, azalea::BlockXZ(0, 0))
                ).await;
                let chest = bot.open_chest(&bot.world.find_one_block(|b| b.id == "minecraft:chest")).await.unwrap();
                bot.take_amount(&chest, 3, |i| i.id == "#minecraft:planks").await;
                // when rust adds async drop this won't be necessary
                chest.close().await;

                let crafting_table = bot.open_crafting_table(&bot.world.find_one_block(|b| b.id == "minecraft:crafting_table")).await.unwrap();
                bot.craft(&crafting_table, &bot.recipe_for("minecraft:sticks")).await?;
                let pickaxe = bot.craft(&crafting_table, &bot.recipe_for("minecraft:wooden_pickaxe")).await?;
                crafting_table.close().await;

                bot.hold(&pickaxe);
                loop {
                    if let Err(e) = bot.dig(bot.feet_coords().down(1)).await {
                        println!("{:?}", e);
                        break;
                    }
                }
            }
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
```
