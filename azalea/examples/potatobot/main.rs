use azalea::{Account, Client, Event, MoveDirection, pathfinder, Vec3, BlockPos, ItemKind};
use std::{convert::TryInto, sync::{Arc, MutexGuard, Mutex}};

#[derive(Default)]
struct State {
    pub eating: bool,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let account = Account::offline("bot");
    let (bot, mut rx) = account.join(&"localhost".try_into().unwrap()).await.unwrap();
    let state = Arc::new(Mutex::new(State::default()));
    let pathfinder_state = Arc::new(Mutex::new(pathfinder::State::default()));

    // Maybe this (along with state stuff) could be turned into a macro in
    // the future?
    while let Some(event) = rx.recv().await {
        // You must do this for every plugin. If you want to disable a plugin,
        // simply don't call its event handler.
        tokio::spawn(async {
            autoeat_handle(bot, event, state.clone()).await;
            pathfinder::handle(bot, event, pathfinder_state.clone()).await;
            handle(bot.clone(), event, state.clone()).await;
        });
    }
}

async fn handle(state: State, bot: Client, event: Event) -> anyhow::Result<()> {
    match event {
        Event::Login => {
            goto_farm(state, bot).await?;
            // after we get to the farm, start farming

        },
        Event::Tick => {
        },
        Event::Packet(_) => {},
        _ => {}
    }

    Ok(())
}


// go to the place where we start farming
async fn goto_farm(state: State, bot: Client, event: Event) -> anyhow::Result<()> {
    bot.state.goto(
        pathfinder::Goals::Near(5, BlockPos::new(0, 70, 0))
    ).await?;
    Ok(())
}

async fn autoeat_handle(state: Arc<Mutex<State>>, bot: &mut , event: Event) {
    match event {
        Event::UpdateHunger => {
            if !bot.using_held_item() && bot.food_level() <= 17 {
                if let azalea::Slot::Present(_) = bot.hold(azalea::ItemGroup::Food).await {
                    bot.use_held_item().await;
                }
            }
        }
    }
}


#[derive(Default, Clone)]
enum FarmTask {
    #[default]
    GoToFarm,
    Farm,
    Deposit 
}


// go to the chest and deposit everything in our inventory.
async fn deposit(bot: &mut Client, state: &mut State) -> anyhow::Result<()> {
    // first throw away any garbage we might have
    bot.toss(
        |item| item.kind != ItemKind::Potato && item.kind != ItemKind::DiamondHoe
    );

    bot.state.goto(Vec3::new ( 0, 70, 0 )).await?;
    let chest = bot.open_container(&bot.world.block_at(BlockPos::new(0, 70, 0)).await.unwrap();

    let inventory_potato_count: usize = bot.inventory().count_total(|item| item.kind == ItemKind::Potato);;
    if inventory_potato_count > 64 {
        chest.deposit_total_count(
            |item| item.kind == azalea::ItemKind::Potato,
            inventory_potato_count - 64
        ).await;
    }
    chest.close().await;
    Ok(())
}
