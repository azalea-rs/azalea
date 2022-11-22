use azalea::pathfinder::BlockPosGoal;
use azalea::{prelude::*, BlockPos, Swarm, SwarmEvent};
use azalea::{Account, Client, Event};
use std::time::Duration;

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
            address: "localhost",

            states: states.clone(),
            swarm_state: SwarmState::default(),

            plugins: plugins![],
            swarm_plugins: swarm_plugins![],

            handle,
            swarm_handle,

            // join_delay: Some(Duration::from_millis(100)),
            join_delay: None,
        })
        .await;
        println!("{e:?}");
    }
}

async fn handle(bot: Client, event: Event, _state: State) -> anyhow::Result<()> {
    match event {
        Event::Login => {
            bot.chat("Hello world").await?;
        }
        Event::Chat(m) => {
            if m.content() == bot.profile.name {
                bot.chat("Bye").await?;
                tokio::time::sleep(Duration::from_millis(50)).await;
                bot.disconnect().await?;
            }
            if m.message().to_string() == "<py5> goto" {
                let entity = bot
                    .world
                    .read()
                    .entity_by_uuid(&uuid::uuid!("6536bfed-8695-48fd-83a1-ecd24cf2a0fd"));
                println!("entity: {:?}", entity);
                if let Some(entity) = entity {
                    let target_pos_vec3 = entity.pos();
                    let target_pos: BlockPos = target_pos_vec3.into();
                    println!("target_pos: {:?}", target_pos);
                    bot.goto(BlockPosGoal::from(target_pos));
                }
            }
        }
        Event::Initialize => {
            println!("initialized");
        }
        _ => {}
    }

    Ok(())
}

async fn swarm_handle(
    mut swarm: Swarm<State>,
    event: SwarmEvent,
    _state: SwarmState,
) -> anyhow::Result<()> {
    match &event {
        SwarmEvent::Disconnect(account) => {
            println!("bot got kicked! {}", account.username);
            tokio::time::sleep(Duration::from_secs(5)).await;
            swarm.add(account, State::default()).await?;
        }
        SwarmEvent::Chat(m) => {
            println!("swarm chat message: {}", m.message().to_ansi(None));
        }
        _ => {}
    }
    Ok(())
}
