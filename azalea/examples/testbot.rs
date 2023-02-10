//! a bot for testing new azalea features

#![feature(type_alias_impl_trait)]

use azalea::ecs::query::With;
use azalea::entity::metadata::Player;
use azalea::entity::Position;
use azalea::pathfinder::BlockPosGoal;
use azalea::{prelude::*, swarm::prelude::*, BlockPos, GameProfileComponent, WalkDirection};
use azalea::{Account, Client, Event};
use azalea_protocol::packets::game::serverbound_client_command_packet::ServerboundClientCommandPacket;
use std::time::Duration;

#[derive(Default, Clone, Component)]
struct State {}

#[derive(Default, Clone, Resource)]
struct SwarmState {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    {
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

    for i in 0..1 {
        accounts.push(Account::offline(&format!("bot{i}")));
        states.push(State::default());
    }

    loop {
        let e = SwarmBuilder::new()
            .add_accounts(accounts.clone())
            .set_handler(handle)
            .set_swarm_handler(swarm_handle)
            .join_delay(Duration::from_millis(1000))
            .start("localhost")
            .await;
        // let e = azalea::ClientBuilder::new()
        //     .set_handler(handle)
        //     .start(Account::offline("bot"), "localhost")
        //     .await;
        eprintln!("{e:?}");
    }
}

async fn handle(mut bot: Client, event: Event, _state: State) -> anyhow::Result<()> {
    match event {
        Event::Init => {
            println!("bot init");
            // bot.set_client_information(ClientInformation {
            //     view_distance: 2,
            //     ..Default::default()
            // })
            // .await?;
        }
        Event::Login => {
            bot.chat("Hello world");
        }
        Event::Chat(m) => {
            println!("client chat message: {}", m.content());
            if m.content() == bot.profile.name {
                bot.chat("Bye");
                tokio::time::sleep(Duration::from_millis(50)).await;
                bot.disconnect();
            }
            let Some(sender) = m.username() else {
                return Ok(())
            };
            // let mut ecs = bot.ecs.lock();
            // let entity = bot
            //     .ecs
            //     .lock()
            //     .query::<&Player>()
            //     .iter(&mut ecs)
            //     .find(|e| e.name() == Some(sender));
            // let entity = bot.entity_by::<With<Player>>(|name: &Name| name == sender);
            let entity = bot.entity_by::<With<Player>, (&GameProfileComponent,)>(
                |profile: &&GameProfileComponent| {
                    println!("entity {profile:?}");
                    profile.name == sender
                },
            );
            println!("sender entity: {entity:?}");
            if let Some(entity) = entity {
                match m.content().as_str() {
                    "whereami" => {
                        let pos = bot.entity_component::<Position>(entity);
                        bot.chat(&format!("You're at {pos:?}",));
                    }
                    "whereareyou" => {
                        let pos = bot.component::<Position>();
                        bot.chat(&format!("I'm at {pos:?}",));
                    }
                    "goto" => {
                        let entity_pos = bot.entity_component::<Position>(entity);
                        let target_pos: BlockPos = entity_pos.into();
                        println!("going to {target_pos:?}");
                        bot.goto(BlockPosGoal::from(target_pos));
                    }
                    "look" => {
                        let entity_pos = bot.entity_component::<Position>(entity);
                        let target_pos: BlockPos = entity_pos.into();
                        println!("target_pos: {target_pos:?}");
                        bot.look_at(target_pos.center());
                    }
                    "jump" => {
                        bot.set_jumping(true);
                    }
                    "walk" => {
                        bot.walk(WalkDirection::Forward);
                    }
                    "stop" => {
                        bot.set_jumping(false);
                        bot.walk(WalkDirection::None);
                    }
                    "lag" => {
                        std::thread::sleep(Duration::from_millis(1000));
                    }
                    _ => {}
                }
            }
        }
        Event::Death(_) => {
            bot.write_packet(ServerboundClientCommandPacket {
                action: azalea_protocol::packets::game::serverbound_client_command_packet::Action::PerformRespawn,
            }.get());
        }
        _ => {}
    }

    Ok(())
}

async fn swarm_handle(
    mut swarm: Swarm,
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
            println!("swarm chat message: {}", m.message().to_ansi());
            if m.message().to_string() == "<py5> world" {
                for (name, world) in &swarm.world_container.read().worlds {
                    println!("world name: {name}");
                    if let Some(w) = world.upgrade() {
                        for chunk_pos in w.read().chunks.chunks.values() {
                            println!("chunk: {chunk_pos:?}");
                        }
                    } else {
                        println!("nvm world is gone");
                    }
                }
            }
            if m.message().to_string() == "<py5> hi" {
                for bot in swarm {
                    bot.chat("hello");
                }
            }
        }
        _ => {}
    }
    Ok(())
}
