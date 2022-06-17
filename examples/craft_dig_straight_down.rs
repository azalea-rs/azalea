use azalea::{Bot, Event};

let bot = Bot::offline("bot");
// or let bot = azalea::Bot::microsoft("access token").await;

bot.join("localhost".try_into().unwrap()).await.unwrap();

loop {
    match bot.next().await {
        Event::Message(m) {
            if m.username == bot.username { return };
            if m.message = "go" {
                bot.goto_goal(
                    pathfinder::Goals::NearXZ(5, azalea::BlockXZ(0, 0))
                ).await;
                let chest = bot.open_chest(&bot.world.find_one_block(|b| b.id == "minecraft:chest")).await.unwrap();
                bot.take_amount(&chest, 5, |i| i.id == "#minecraft:planks").await;
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
