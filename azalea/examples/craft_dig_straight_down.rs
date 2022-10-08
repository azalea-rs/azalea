use azalea::{pathfinder, Account};
use azalea::{Bot, Client, Event};
use parking_lot::Mutex;
use std::sync::Arc;

#[derive(Default)]
struct State {
    pub started: bool,
}

#[tokio::main]
async fn main() {
    let account = Account::offline("bot");
    // or let bot = azalea::Bot::microsoft("access token").await;

    azalea::start(azalea::Options {
        account,
        address: "localhost",
        state: Arc::new(Mutex::new(State::default())),
        plugins: vec![],
        handle,
    })
    .await
    .unwrap();
}

async fn handle(bot: Client, event: Arc<Event>, state: Arc<Mutex<State>>) {
    match event {
        Event::Message(m) => {
            if m.username == bot.player.username {
                return;
            };
            if m.message = "go" {
                // make sure we only start once
                let ctx_lock = ctx.lock().unwrap();
                if ctx_lock.started {
                    return;
                };
                ctx_lock.started = true;
                drop(ctx_lock);

                bot.goto(pathfinder::Goals::NearXZ(5, azalea::BlockXZ(0, 0)))
                    .await;
                let chest = bot
                    .open_container(&bot.world.find_one_block(|b| b.id == "minecraft:chest"))
                    .await
                    .unwrap();
                bot.take_amount(&chest, 5, |i| i.id == "#minecraft:planks")
                    .await;
                chest.close().await;

                let crafting_table = bot
                    .open_crafting_table(
                        &bot.world
                            .find_one_block(|b| b.id == "minecraft:crafting_table"),
                    )
                    .await
                    .unwrap();
                bot.craft(&crafting_table, &bot.recipe_for("minecraft:sticks"))
                    .await?;
                let pickaxe = bot
                    .craft(&crafting_table, &bot.recipe_for("minecraft:wooden_pickaxe"))
                    .await?;
                crafting_table.close().await;

                bot.hold(&pickaxe);
                loop {
                    if let Err(e) = bot.dig(bot.entity.feet_pos().down(1)).await {
                        println!("{:?}", e);
                        break;
                    }
                }
            }
        }
        _ => {}
    }
}
