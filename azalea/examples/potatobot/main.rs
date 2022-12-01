mod autoeat;
use azalea::prelude::*;
use azalea::{pathfinder, BlockPos, ItemKind, Vec3};

#[derive(Default, Clone)]
struct State {}

#[tokio::main]
async fn main() {
    env_logger::init();

    let account = Account::offline("bot");

    azalea::start(azalea::Options {
        account,
        address: "localhost",
        state: State::default(),
        plugins: plugins![autoeat::Plugin, pathfinder::Plugin],
        handle,
    })
    .await
    .unwrap();
}

async fn handle(bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    match event {
        Event::Login => {
            goto_farm(bot, state).await?;
            // after we get to the farm, start farming
            farm(bot, state).await?;
        }
        _ => {}
    }

    Ok(())
}

// go to the place where we start farming
async fn goto_farm(bot: Client, state: State) -> anyhow::Result<()> {
    bot.goto(pathfinder::Goals::Near(5, BlockPos::new(0, 70, 0)))
        .await?;
    Ok(())
}

// go to the chest and deposit everything in our inventory.
async fn deposit(bot: &mut Client, state: State) -> anyhow::Result<()> {
    // first throw away any garbage we might have
    bot.toss(|item| item.kind != ItemKind::Potato && item.kind != ItemKind::DiamondHoe);

    bot.goto(Vec3::new(0, 70, 0)).await?;
    let chest = bot
        .open_container(&bot.world.block_at(BlockPos::new(0, 70, 0)))
        .await
        .unwrap();

    let inventory_potato_count: usize = bot
        .inventory()
        .count_total(|item| item.kind == ItemKind::Potato);
    if inventory_potato_count > 64 {
        chest
            .deposit_total_count(
                |item| item.kind == azalea::ItemKind::Potato,
                inventory_potato_count - 64,
            )
            .await;
    }
    chest.close().await;
    Ok(())
}
