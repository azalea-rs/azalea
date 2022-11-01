use azalea::{prelude::*, BlockPos};
use azalea::{Account, Client, Event};
use azalea_pathfinder::{BlockPosGoal, Trait};

#[derive(Default, Clone)]
struct State {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let account = Account::microsoft("example@example.com").await?;

    azalea::start(azalea::Options {
        account,
        address: "localhost",
        state: State::default(),
        plugins: vec![Box::new(azalea_pathfinder::Plugin::default())],
        handle,
    })
    .await
    .unwrap();

    Ok(())
}

async fn handle(bot: Client, event: Event, _state: State) -> anyhow::Result<()> {
    match event {
        Event::Login => {
            bot.chat("Hello world").await?;
            bot.goto(BlockPosGoal::from(BlockPos::new(0, -60, 12)));
        }
        Event::Initialize => {
            println!("initialized");
        }
        Event::Tick => {
            // bot.jump();
        }
        _ => {}
    }

    Ok(())
}
