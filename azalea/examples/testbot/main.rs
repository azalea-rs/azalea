//! A relatively simple bot for demonstrating some of Azalea's capabilities.
//!
//! ## Usage
//!
//! - Modify the consts below if necessary.
//! - Run `cargo r --example testbot -- [arguments]`. (see below)
//! - Commands are prefixed with `!` in chat. You can send them either in public
//!   chat or as a /msg.
//! - Some commands to try are `!goto`, `!killaura true`, `!down`. Check the
//!   `commands` directory to see all of them.
//!
//! ### Arguments
//!
//! - `--owner` or `-O`: The username of the player who owns the bot. The bot
//!   will ignore commands from other players.
//! - `--account` or `-A`: The username or email of the bot.
//! - `--server` or `-S`: The address of the server to join.
//! - `--pathfinder-debug-particles` or `-P`: Whether the bot should run
//!   /particle a ton of times to show where it's pathfinding to. You should
//!   only have this on if the bot has operator permissions, otherwise it'll
//!   just spam the server console unnecessarily.

#![feature(trivial_bounds)]

mod commands;
pub mod killaura;

use std::time::Duration;
use std::{env, process};
use std::{sync::Arc, thread};

use azalea::ClientInformation;
use azalea::brigadier::command_dispatcher::CommandDispatcher;
use azalea::ecs::prelude::*;
use azalea::pathfinder::debug::PathfinderDebugParticles;
use azalea::prelude::*;
use azalea::swarm::prelude::*;
use commands::{CommandSource, register_commands};
use parking_lot::Mutex;

#[tokio::main]
async fn main() {
    let args = parse_args();

    thread::spawn(deadlock_detection_thread);

    let join_address = args.server.clone();

    let mut builder = SwarmBuilder::new()
        .set_handler(handle)
        .set_swarm_handler(swarm_handle);

    for username_or_email in &args.accounts {
        let account = if username_or_email.contains('@') {
            Account::microsoft(username_or_email).await.unwrap()
        } else {
            Account::offline(username_or_email)
        };

        builder = builder.add_account_with_state(account, State::new());
    }

    let mut commands = CommandDispatcher::new();
    register_commands(&mut commands);

    builder
        .join_delay(Duration::from_millis(100))
        .set_swarm_state(SwarmState {
            args,
            commands: Arc::new(commands),
        })
        .start(join_address)
        .await
        .unwrap();
}

/// Runs a loop that checks for deadlocks every 10 seconds.
///
/// Note that this requires the `deadlock_detection` parking_lot feature to be
/// enabled, which is only enabled in azalea by default when running in debug
/// mode.
fn deadlock_detection_thread() {
    loop {
        thread::sleep(Duration::from_secs(10));
        let deadlocks = parking_lot::deadlock::check_deadlock();
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
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum BotTask {
    #[default]
    None,
}

#[derive(Component, Clone, Default)]
pub struct State {
    pub killaura: bool,
    pub task: Arc<Mutex<BotTask>>,
}

impl State {
    fn new() -> Self {
        Self {
            killaura: true,
            task: Arc::new(Mutex::new(BotTask::None)),
        }
    }
}

#[derive(Resource, Default, Clone)]
struct SwarmState {
    pub args: Args,
    pub commands: Arc<CommandDispatcher<Mutex<CommandSource>>>,
}

async fn handle(bot: Client, event: azalea::Event, state: State) -> anyhow::Result<()> {
    let swarm = bot.resource::<SwarmState>();

    match event {
        azalea::Event::Init => {
            bot.set_client_information(ClientInformation {
                view_distance: 32,
                ..Default::default()
            })
            .await;
            if swarm.args.pathfinder_debug_particles {
                bot.ecs
                    .lock()
                    .entity_mut(bot.entity)
                    .insert(PathfinderDebugParticles);
            }
        }
        azalea::Event::Chat(chat) => {
            let (Some(username), content) = chat.split_sender_and_content() else {
                return Ok(());
            };
            if username != swarm.args.owner_username {
                return Ok(());
            }

            println!("{:?}", chat.message());

            let command = if chat.is_whisper() {
                Some(content)
            } else {
                content.strip_prefix('!').map(|s| s.to_owned())
            };
            if let Some(command) = command {
                match swarm.commands.execute(
                    command,
                    Mutex::new(CommandSource {
                        bot: bot.clone(),
                        chat: chat.clone(),
                        state: state.clone(),
                    }),
                ) {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("{err:?}");
                        let command_source = CommandSource {
                            bot,
                            chat: chat.clone(),
                            state: state.clone(),
                        };
                        command_source.reply(&format!("{err:?}"));
                    }
                }
            }
        }
        azalea::Event::Tick => {
            killaura::tick(bot.clone(), state.clone())?;

            let task = *state.task.lock();
            match task {
                BotTask::None => {}
            }
        }
        _ => {}
    }

    Ok(())
}
async fn swarm_handle(_swarm: Swarm, event: SwarmEvent, _state: SwarmState) -> anyhow::Result<()> {
    match &event {
        SwarmEvent::Disconnect(account, _join_opts) => {
            println!("bot got kicked! {}", account.username);
        }
        SwarmEvent::Chat(chat) => {
            if chat.message().to_string() == "The particle was not visible for anybody" {
                return Ok(());
            }
            println!("{}", chat.message().to_ansi());
        }
        _ => {}
    }

    Ok(())
}

#[derive(Debug, Clone, Default)]
pub struct Args {
    pub owner_username: String,
    pub accounts: Vec<String>,
    pub server: String,
    pub pathfinder_debug_particles: bool,
}

fn parse_args() -> Args {
    let mut owner_username = "admin".to_string();
    let mut accounts = Vec::new();
    let mut server = "localhost".to_string();
    let mut pathfinder_debug_particles = false;

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--owner" | "-O" => {
                owner_username = args.next().expect("Missing owner username");
            }
            "--account" | "-A" => {
                for account in args.next().expect("Missing account").split(',') {
                    accounts.push(account.to_string());
                }
            }
            "--server" | "-S" => {
                server = args.next().expect("Missing server address");
            }
            "--pathfinder-debug-particles" | "-P" => {
                pathfinder_debug_particles = true;
            }
            _ => {
                eprintln!("Unknown argument: {}", arg);
                process::exit(1);
            }
        }
    }

    if accounts.is_empty() {
        accounts.push("azalea".to_string());
    }

    Args {
        owner_username,
        accounts,
        server,
        pathfinder_debug_particles,
    }
}
