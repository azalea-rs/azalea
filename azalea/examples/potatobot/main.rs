mod autoeat;

use azalea::prelude::*;
use azalea::{pathfinder, Account, BlockPos, Client, Event, ItemKind, MoveDirection, Plugin, Vec3};
use parking_lot::Mutex;
use std::sync::Arc;

#[derive(Default)]
struct State {}

#[tokio::main]
async fn main() {
    env_logger::init();

    let account = Account::offline("bot");

    azalea::start(azalea::Options {
        account,
        address: "localhost",
        state: Arc::new(Mutex::new(State::default())),
        plugins: vec![
            Arc::new(autoeat::Plugin::default()),
            Arc::new(pathfinder::Plugin::default()),
        ],
        handle,
    })
    .await
    .unwrap();
}

async fn handle(bot: Client, event: Arc<Event>, state: Arc<Mutex<State>>) -> anyhow::Result<()> {
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
async fn goto_farm(bot: Client, state: Arc<Mutex<State>>) -> anyhow::Result<()> {
    bot.goto(pathfinder::Goals::Near(5, BlockPos::new(0, 70, 0)))
        .await?;
    Ok(())
}

// go to the chest and deposit everything in our inventory.
async fn deposit(bot: &mut Client, state: &mut Arc<Mutex<State>>) -> anyhow::Result<()> {
    // first throw away any garbage we might have
    bot.toss(|item| item.kind != ItemKind::Potato && item.kind != ItemKind::DiamondHoe);

    bot.goto(Vec3::new(0, 70, 0)).await?;
    let chest = bot
        .open_container(&bot.dimension.block_at(BlockPos::new(0, 70, 0)))
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
