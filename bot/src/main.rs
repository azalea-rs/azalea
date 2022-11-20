use azalea::pathfinder::BlockPosGoal;
use azalea::{prelude::*, BlockPos, Swarm, SwarmEvent, WalkDirection};
use azalea::{Account, Client, Event};
use parking_lot::Mutex;
use rand::Rng;
use std::sync::Arc;

#[derive(Default, Clone)]
struct State {
    moving: Arc<Mutex<bool>>,
}

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
                println!("Deadlock #{i}");
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
            address: "92.222.245.59",

            states: states.clone(),
            swarm_state: SwarmState::default(),

            plugins: plugins![],
            swarm_plugins: swarm_plugins![],

            handle,
            swarm_handle,

            join_delay: Some(std::time::Duration::from_secs(5)),
        })
        .await;
        println!("{e:?}");
    }
}

async fn handle(mut bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    match event {
        Event::Login => {
            bot.chat("Hello world").await?;
        }
        Event::Chat(m) => {
            // println!("{}", m.message().to_ansi(None));
            // if m.message().to_string() == "<py5> goto" {
            //     let target_pos_vec3 = bot
            //         .world
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
            // look in a random direction and walk for 1-3 seconds
            {
                let mut moving = state.moving.lock();
                if *moving {
                    return Ok(());
                }
                *moving = true;
            }

            let (rotation, duration) = {
                let mut rng = rand::thread_rng();
                (rng.gen_range(0.0..360.0), rng.gen_range(1..3))
            };

            bot.set_rotation(rotation, 0.);
            bot.walk(WalkDirection::Forward);
            tokio::time::sleep(tokio::time::Duration::from_secs(duration)).await;
            bot.walk(WalkDirection::None);
            *state.moving.lock() = false;
            // bot.jump();
        }
        _ => {}
    }

    Ok(())
}

async fn swarm_handle<S>(
    mut _swarm: Swarm<S>,
    _event: SwarmEvent,
    _state: SwarmState,
) -> anyhow::Result<()> {
    Ok(())
}
