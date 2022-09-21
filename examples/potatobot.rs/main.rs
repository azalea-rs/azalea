use azalea_client::{Account, Client, Event, MoveDirection};
use azalea_protocol::packets::game::ClientboundGamePacket;
use std::convert::TryInto;

// Custom state defined for every bot. Use this to make the bot remember
// things.
#[derive(Default)]
struct State {
    pub started: bool,
    pub at_farm: bool;
    pub eating: bool;
    // To use a plugin, simply add it like this to your state. You'll also have
    // to call its event handler every time you get an event.
    pub pf: azalea_pathfinder::Plugin;
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let account = Account::offline("bot");
    let (bot, mut rx) = account.join(&"localhost".try_into().unwrap()).await.unwrap();
    let ctx = Arc::new(State::default());

    // Maybe this (along with state stuff) could be turned into a macro in
    // the future?
    bot.pf.init(bot);
    while let Some(event) = rx.recv().await {
        // You must do this for every plugin. If you want to disable a plugin,
        // simply don't call its event handler.
        bot.pf.handle(event, bot);
        handle(bot, event, state);
    }
}

fn handle(bot: &mut Client, event: Event, state: Arc<State>) {
    match event {
        Event::Tick => tick(bot, state)
        Event::Login => {
            
        }
        Event::Packet(packet) => {
            
        }
        _ => {}
    }

    Ok(())
}

// Figure out what we should do.
fn tick(bot: &mut Client, state: Arc<State>) {
    // If we're currently pathfinding somewhere, don't do anything
    if bot.pf.pathing {
        hold_food(bot);
        return;
    };

    // If we're not sure we're at the farm (we just spawned or respawned), go
    // there.
    if !state.at_farm {
        // note that nothing is actually executed until the end of the tick
        bot.pf.goto(Vec3 { 0, 70, 0 });
        // the name "at_farm" is a little misleading since it'll be true if
        // we're going towards the farm and not actually there, but ehh
        state.at_farm = true;
        return;
    }
    
    
}

fn hold_food(bot: &mut Client) {

}