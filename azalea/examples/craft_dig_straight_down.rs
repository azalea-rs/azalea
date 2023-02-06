use azalea::pathfinder;
use azalea::prelude::*;
use parking_lot::Mutex;
use std::sync::Arc;

#[derive(Default, Clone, Component)]
struct State {
    pub started: Arc<Mutex<bool>>,
}

#[tokio::main]
async fn main() {
    let account = Account::offline("bot");
    // or let bot = Account::microsoft("email").await;

    azalea::ClientBuilder::new()
        .set_handler(handle)
        .start(account, "localhost")
        .await
        .unwrap();
}

async fn handle(bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    match event {
        Event::Chat(m) => {
            if m.username() == Some(bot.profile.name) {
                return Ok(());
            };
            if m.content() == "go" {
                {
                    // make sure we only start once
                    if *state.started.lock() {
                        return Ok(());
                    };
                    *state.started.lock() = true;
                }

                bot.goto(pathfinder::Goals::NearXZ(5, azalea::BlockXZ(0, 0)))
                    .await;
                let chest = bot
                    .open_container(&bot.world().find_one_block(|b| b.id == "minecraft:chest"))
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
                    if let Err(e) = bot
                        .dig(azalea::entity::feet_pos(bot.entity()).down(1))
                        .await
                    {
                        println!("{:?}", e);
                        break;
                    }
                }
            }
        }
        _ => {}
    }

    Ok(())
}
