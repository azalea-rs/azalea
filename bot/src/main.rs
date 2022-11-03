use azalea::{prelude::*, BlockPos, WalkDirection};
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
        plugins: plugins![azalea_pathfinder::Plugin::default()],
        handle,
    })
    .await
    .unwrap();

    Ok(())
}

async fn handle(mut bot: Client, event: Event, _state: State) -> anyhow::Result<()> {
    match event {
        Event::Login => {
            // bot.chat("Hello world").await?;
        }
        Event::Chat(m) => {
            println!("{}", m.message().to_ansi(None));
            if m.message().to_string() == "<py5> goto" {
                let target_pos_vec3 = bot
                    .dimension
                    .read()
                    .entity_by_uuid(&uuid::uuid!("6536bfed869548fd83a1ecd24cf2a0fd"))
                    .unwrap()
                    .pos()
                    .clone();
                let target_pos: BlockPos = (&target_pos_vec3).into();
                // bot.look_at(&target_pos_vec3);
                bot.goto(BlockPosGoal::from(target_pos));
                // bot.walk(WalkDirection::Forward);
            }
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
