use azalea::pathfinder::BlockPosGoal;
use azalea::{prelude::*, BlockPos, Swarm, SwarmEvent};
use azalea::{Account, Client, Event};

#[derive(Default, Clone)]
struct State {}

#[derive(Default, Clone)]
struct SwarmState {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    {
        // only for #[cfg]
        use parking_lot::deadlock;
        use std::thread;
        use std::time::Duration;

        // Create a background thread which checks for deadlocks every 10s
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(10));
            let deadlocks = deadlock::check_deadlock();
            if deadlocks.is_empty() {
                continue;
            }

            println!("{} deadlocks detected", deadlocks.len());
            for (i, threads) in deadlocks.iter().enumerate() {
                println!("Deadlock #{}", i);
                for t in threads {
                    println!("Thread Id {:#?}", t.thread_id());
                    println!("{:#?}", t.backtrace());
                }
            }
        });
    }

    let mut accounts = Vec::new();
    let mut states = Vec::new();

    for i in 0..5 {
        accounts.push(Account::offline(&format!("bot{}", i)));
        states.push(State::default());
    }

    loop {
        let e = azalea::start_swarm(azalea::SwarmOptions {
            accounts: accounts.clone(),
            address: "localhost",

            states: states.clone(),
            swarm_state: SwarmState::default(),

            plugins: plugins![],
            swarm_plugins: swarm_plugins![],

            handle,
            swarm_handle,
        })
        .await;
        println!("{:?}", e);
    }
}

async fn handle(bot: Client, event: Event, _state: State) -> anyhow::Result<()> {
    match event {
        Event::Login => {
            bot.chat("Hello world").await?;
        }
        Event::Chat(m) => {
            // println!("{}", m.message().to_ansi(None));
            // if m.message().to_string() == "<py5> goto" {
            //     let target_pos_vec3 = bot
            //         .dimension
            //         .read()
            //         .entity_by_uuid(&uuid::uuid!("6536bfed869548fd83a1ecd24cf2a0fd"))
            //         .unwrap()
            //         .pos()
            //         .clone();
            //     let target_pos: BlockPos = (&target_pos_vec3).into();
            //     // bot.look_at(&target_pos_vec3);
            //     bot.goto(BlockPosGoal::from(target_pos));
            //     // bot.walk(WalkDirection::Forward);
            // }
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

async fn swarm_handle(
    mut _swarm: Swarm,
    _event: SwarmEvent,
    _state: SwarmState,
) -> anyhow::Result<()> {
    Ok(())
}
