mod autoeat;

use azalea::{pathfinder, Account, BlockPos, Client, Event, ItemKind, MoveDirection, Vec3};
use std::{
    convert::TryInto,
    sync::{Arc, Mutex},
};

#[derive(Default)]
struct State {
    pub eating: bool,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let account = Account::offline("bot");
    let (bot, mut rx) = account
        .join(&"localhost".try_into().unwrap())
        .await
        .unwrap();

    // Maybe all this could be turned into a macro in the future?
    let state = Arc::new(Mutex::new(State::default()));
    let autoeat_state = Arc::new(Mutex::new(autoeat::State::default()));
    let pathfinder_state = Arc::new(Mutex::new(pathfinder::State::default()));
    while let Some(event) = rx.recv().await {
        // we put it into an Arc so it's cheaper to clone
        let event = Arc::new(event);

        tokio::spawn(autoeat::handle(
            bot.clone(),
            event.clone(),
            autoeat_state.clone(),
        ));
        tokio::spawn(pathfinder::handle(
            bot.clone(),
            event.clone(),
            pathfinder_state.clone(),
        ));
        tokio::spawn(handle(bot.clone(), event.clone(), state.clone()));
    }
}

async fn handle(bot: Client, event: Event, state: Arc<Mutex<State>>) -> anyhow::Result<()> {
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
    bot.state
        .goto(pathfinder::Goals::Near(5, BlockPos::new(0, 70, 0)))
        .await?;
    Ok(())
}

// go to the chest and deposit everything in our inventory.
async fn deposit(bot: &mut Client, state: &mut Arc<Mutex<State>>) -> anyhow::Result<()> {
    // first throw away any garbage we might have
    bot.toss(|item| item.kind != ItemKind::Potato && item.kind != ItemKind::DiamondHoe);

    bot.state.goto(Vec3::new(0, 70, 0)).await?;
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
