use azalea::{Account, Client, Event, MoveDirection, pathfinder, Vec3};
use std::convert::TryInto;

// Custom state defined for every bot. Use this to make the bot remember
// things.
#[derive(Default)]
struct State {
    pub farm_task: FarmTask,
    pub eating: bool,
    // To use a plugin, simply add it like this to your state. You'll also have
    // to call its event handler every time you get an event.
    pub pf: pathfinder::Plugin,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let account = Account::offline("bot");
    let (bot, mut rx) = account.join(&"localhost".try_into().unwrap()).await.unwrap();
    let state = Arc::new(State::default());

    // Maybe this (along with state stuff) could be turned into a macro in
    // the future?
    while let Some(event) = rx.recv().await {
        // You must do this for every plugin. If you want to disable a plugin,
        // simply don't call its event handler.
        bot.pf.handle(event, bot);
        handle(bot, event, &mut state);
    }
}

fn handle(bot: &mut Client, event: Event, state: &mut State) {
    match event {
        Event::Tick => tick(bot, state),
        Event::Login => {},
        Event::Packet(packet) => {},
        _ => {}
    }
}

// Figure out what we should do.
fn tick(bot: &mut Client, state: &mut State) {
    // If we're currently pathfinding somewhere, don't do anything (except eat maybe)
    if state.pf.pathing {
        tick_eat(bot, state);
        return;
    };
    if let FarmTask::Depositing { .. } = state.farm_task {
        tick_deposit(bot, state);
        return;
    }

    // If we're not sure we're at the farm (we just spawned or respawned), go
    // there.
    if !state.at_farm {
        // note that nothing is actually executed until the end of the tick
        bot.state.goto(pathfinder::Goals::Near(5, azalea::BlockPos::new(0, 70, 0)));
        // the name "at_farm" is a little misleading since it'll be true if
        // we're going towards the farm and not actually there, but ehh
        state.at_farm = true;
        return;
    }
    
    tick_farm(bot, state);
}

/// Eat something if we're hungry.
fn tick_eat(bot: &mut Client, state: &mut State) {
    if !bot.using_held_item() && bot.food_level() <= 17 {
        if let Slot::Present(_) = bot.hold(azalea::ItemGroup::Food) {
            bot.use_held_item();
        }
    }
}

#[derive(Default)]
enum FarmTask {
    #[default]
    GoingToFarm,
    Depositing { at_deposit_area: bool }
}

fn tick_farm(bot: &mut Client, state: &mut State) {
    if bot.inventory.is_full() {
    }
}

// go to the chest and deposit everything in our inventory.
fn tick_deposit(bot: &mut Client, state: &mut State) {
    let FarmTask::Depositing { at_deposit_area } = state.farm_task;
    
    if !at_deposit_area {
        bot.state.goto(Vec3::new ( 0, 70, 0 ));
        let chest = bot.open_container(&bot.world.block_at(Vec3 { 0, 70, 0 }).await.unwrap();
    };

    for item in bot.inventory().items() {
        chest.deposit(item);
    }
    chest.close();
    return;
}
