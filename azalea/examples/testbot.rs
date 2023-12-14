//! a bot for testing new azalea features

use azalea::ecs::query::With;
use azalea::entity::{metadata::Player, EyeHeight, Position};
use azalea::interact::HitResultComponent;
use azalea::inventory::ItemSlot;
use azalea::pathfinder::goals::BlockPosGoal;
use azalea::{prelude::*, swarm::prelude::*, BlockPos, GameProfileComponent, WalkDirection};
use azalea::{Account, Client, Event};
use azalea_client::{InstanceHolder, SprintDirection};
use azalea_core::position::{ChunkBlockPos, ChunkPos, Vec3};
use azalea_protocol::packets::game::ClientboundGamePacket;
use azalea_world::heightmap::HeightmapKind;
use azalea_world::{InstanceName, MinecraftEntityId};
use std::time::Duration;

#[derive(Default, Clone, Component)]
struct State {}

#[derive(Default, Clone, Resource)]
struct SwarmState {}

#[tokio::main]
async fn main() {
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

    for i in 0..1 {
        accounts.push(Account::offline(&format!("bot{i}")));
    }

    SwarmBuilder::new()
        .add_accounts(accounts.clone())
        .set_handler(handle)
        .set_swarm_handler(swarm_handle)
        .join_delay(Duration::from_millis(100))
        .start("localhost")
        .await
        .unwrap();
}

async fn handle(mut bot: Client, event: Event, _state: State) -> anyhow::Result<()> {
    match event {
        Event::Init => {
            // bot.set_client_information(azalea_client::ClientInformation {
            //     view_distance: 2,
            //     ..Default::default()
            // })
            // .await?;
        }
        Event::Login => {
            bot.chat("Hello world");
        }
        Event::Chat(m) => {
            // println!("client chat message: {}", m.content());
            if m.content() == bot.profile.name {
                bot.chat("Bye");
                tokio::time::sleep(Duration::from_millis(50)).await;
                bot.disconnect();
            }
            let Some(sender) = m.username() else {
                return Ok(());
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
                |(profile,): &(&GameProfileComponent,)| profile.name == sender,
            );
            match m.content().as_str() {
                "whereami" => {
                    let Some(entity) = entity else {
                        bot.chat("I can't see you");
                        return Ok(());
                    };
                    let pos = bot.entity_component::<Position>(entity);
                    bot.chat(&format!("You're at {pos:?}"));
                }
                "whereareyou" => {
                    let pos = bot.position();
                    bot.chat(&format!("I'm at {pos:?}"));
                }
                "goto" => {
                    let Some(entity) = entity else {
                        bot.chat("I can't see you");
                        return Ok(());
                    };
                    let entity_pos = bot.entity_component::<Position>(entity);
                    let target_pos: BlockPos = entity_pos.into();
                    println!("going to {target_pos:?}");
                    bot.goto(BlockPosGoal(target_pos));
                }
                "worldborder" => {
                    bot.goto(BlockPosGoal(BlockPos::new(30_000_000, 70, 0)));
                }
                "look" => {
                    let Some(entity) = entity else {
                        bot.chat("I can't see you");
                        return Ok(());
                    };
                    let entity_pos = bot
                        .entity_component::<Position>(entity)
                        .up(bot.entity_component::<EyeHeight>(entity).into());
                    println!("entity_pos: {entity_pos:?}");
                    bot.look_at(entity_pos);
                }
                "jump" => {
                    bot.set_jumping(true);
                }
                "walk" => {
                    bot.walk(WalkDirection::Forward);
                }
                "sprint" => {
                    bot.sprint(SprintDirection::Forward);
                }
                "stop" => {
                    bot.set_jumping(false);
                    bot.walk(WalkDirection::None);
                }
                "lag" => {
                    std::thread::sleep(Duration::from_millis(1000));
                }
                "quit" => {
                    bot.disconnect();
                    tokio::time::sleep(Duration::from_millis(1000)).await;
                    std::process::exit(0);
                }
                "inventory" => {
                    println!("inventory: {:?}", bot.menu());
                }
                "findblock" => {
                    let target_pos = bot.world().read().find_block(
                        bot.position(),
                        &azalea::registry::Block::DiamondBlock.into(),
                    );
                    bot.chat(&format!("target_pos: {target_pos:?}",));
                }
                "gotoblock" => {
                    let target_pos = bot.world().read().find_block(
                        bot.position(),
                        &azalea::registry::Block::DiamondBlock.into(),
                    );
                    if let Some(target_pos) = target_pos {
                        // +1 to stand on top of the block
                        bot.goto(BlockPosGoal(target_pos.up(1)));
                    } else {
                        bot.chat("no diamond block found");
                    }
                }
                "mineblock" => {
                    let target_pos = bot.world().read().find_block(
                        bot.position(),
                        &azalea::registry::Block::DiamondBlock.into(),
                    );
                    if let Some(target_pos) = target_pos {
                        // +1 to stand on top of the block
                        bot.chat("ok mining diamond block");
                        bot.look_at(target_pos.center());
                        bot.mine(target_pos).await;
                        bot.chat("finished mining");
                    } else {
                        bot.chat("no diamond block found");
                    }
                }
                "lever" => {
                    let target_pos = bot
                        .world()
                        .read()
                        .find_block(bot.position(), &azalea::registry::Block::Lever.into());
                    let Some(target_pos) = target_pos else {
                        bot.chat("no lever found");
                        return Ok(());
                    };
                    bot.goto(BlockPosGoal(target_pos));
                    bot.look_at(target_pos.center());
                    bot.block_interact(target_pos);
                }
                "hitresult" => {
                    let hit_result = bot.get_component::<HitResultComponent>();
                    bot.chat(&format!("hit_result: {hit_result:?}",));
                }
                "chest" => {
                    let target_pos = bot
                        .world()
                        .read()
                        .find_block(bot.position(), &azalea::registry::Block::Chest.into());
                    let Some(target_pos) = target_pos else {
                        bot.chat("no chest found");
                        return Ok(());
                    };
                    bot.look_at(target_pos.center());
                    let container = bot.open_container(target_pos).await;
                    println!("container: {container:?}");
                    if let Some(container) = container {
                        if let Some(contents) = container.contents() {
                            for item in contents {
                                if let ItemSlot::Present(item) = item {
                                    println!("item: {item:?}");
                                }
                            }
                        } else {
                            println!("container was immediately closed");
                        }
                    } else {
                        println!("no container found");
                    }
                }
                "attack" => {
                    let mut nearest_entity = None;
                    let mut nearest_distance = f64::INFINITY;
                    let mut nearest_pos = Vec3::default();
                    let bot_position = bot.position();
                    let bot_entity = bot.entity;
                    let bot_instance_name = bot.component::<InstanceName>();
                    {
                        let mut ecs = bot.ecs.lock();
                        let mut query = ecs.query_filtered::<(
                            azalea::ecs::entity::Entity,
                            &MinecraftEntityId,
                            &Position,
                            &InstanceName,
                            &EyeHeight,
                        ), With<MinecraftEntityId>>();
                        for (entity, &entity_id, position, instance_name, eye_height) in
                            query.iter(&ecs)
                        {
                            if entity == bot_entity {
                                continue;
                            }
                            if instance_name != &bot_instance_name {
                                continue;
                            }

                            let distance = bot_position.distance_to(position);
                            if distance < 4.0 && distance < nearest_distance {
                                nearest_entity = Some(entity_id);
                                nearest_distance = distance;
                                nearest_pos = position.up(**eye_height as f64);
                            }
                        }
                    }
                    if let Some(nearest_entity) = nearest_entity {
                        bot.look_at(nearest_pos);
                        bot.attack(nearest_entity);
                        bot.chat("attacking");
                        let mut ticks = bot.get_tick_broadcaster();
                        while ticks.recv().await.is_ok() {
                            if !bot.has_attack_cooldown() {
                                break;
                            }
                        }
                        bot.chat("finished attacking");
                    } else {
                        bot.chat("no entities found");
                    }
                }
                "heightmap" => {
                    let position = bot.position();
                    let chunk_pos = ChunkPos::from(position);
                    let chunk_block_pos = ChunkBlockPos::from(position);
                    let chunk = bot.world().read().chunks.get(&chunk_pos);
                    if let Some(chunk) = chunk {
                        let heightmaps = &chunk.read().heightmaps;
                        let Some(world_surface_heightmap) =
                            heightmaps.get(&HeightmapKind::WorldSurface)
                        else {
                            bot.chat("no world surface heightmap");
                            return Ok(());
                        };
                        let highest_y = world_surface_heightmap
                            .get_highest_taken(chunk_block_pos.x, chunk_block_pos.z);
                        bot.chat(&format!("highest_y: {highest_y}",));
                    } else {
                        bot.chat("no chunk found");
                    }
                }
                "debugblock" => {
                    // send the block that we're standing on
                    let block_pos = BlockPos::from(bot.position().down(0.1));
                    let block = bot.world().read().get_block_state(&block_pos);
                    bot.chat(&format!("block: {block:?}"));
                }
                "debugchunks" => {
                    {
                        println!("shared:");

                        let mut ecs = bot.ecs.lock();

                        let instance_holder = bot.query::<&InstanceHolder>(&mut ecs).clone();
                        drop(ecs);
                        let local_chunk_storage = &instance_holder.partial_instance.read().chunks;
                        let shared_chunk_storage = instance_holder.instance.read();

                        let mut total_loaded_chunks_count = 0;
                        for (chunk_pos, chunk) in &shared_chunk_storage.chunks.map {
                            if let Some(chunk) = chunk.upgrade() {
                                let in_range = local_chunk_storage.in_range(chunk_pos);
                                println!(
                                    "{chunk_pos:?} has {} references{}",
                                    std::sync::Arc::strong_count(&chunk) - 1,
                                    if in_range { "" } else { " (out of range)" }
                                );
                                total_loaded_chunks_count += 1;
                            }
                        }

                        println!("local:");
                        println!("view range: {}", local_chunk_storage.view_range());
                        println!("view center: {:?}", local_chunk_storage.view_center());

                        let mut local_loaded_chunks_count = 0;
                        for (i, chunk) in local_chunk_storage.chunks().enumerate() {
                            if let Some(chunk) = chunk {
                                let chunk_pos = local_chunk_storage.chunk_pos_from_index(i);
                                println!(
                                    "{chunk_pos:?} (#{i}) has {} references",
                                    std::sync::Arc::strong_count(&chunk)
                                );
                                local_loaded_chunks_count += 1;
                            }
                        }

                        println!("total loaded chunks: {total_loaded_chunks_count}");
                        println!(
                            "local loaded chunks: {local_loaded_chunks_count}/{}",
                            local_chunk_storage.chunks().collect::<Vec<_>>().len()
                        );
                    }
                    {
                        let local_chunk_storage_lock = bot.partial_world();
                        let local_chunk_storage = local_chunk_storage_lock.read();
                        let current_chunk_loaded = local_chunk_storage
                            .chunks
                            .limited_get(&ChunkPos::from(bot.position()));

                        bot.chat(&format!(
                            "current chunk loaded: {}",
                            current_chunk_loaded.is_some()
                        ));
                    }
                }
                _ => {}
            }
        }
        Event::Packet(packet) => {
            if let ClientboundGamePacket::Login(_) = *packet {
                println!("login packet");
            }
        }
        Event::Disconnect(reason) => {
            if let Some(reason) = reason {
                println!("bot got kicked for reason: {}", reason.to_ansi());
            } else {
                println!("bot got kicked");
            }
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
            swarm.add_and_retry_forever(account, State::default()).await;
        }
        SwarmEvent::Chat(m) => {
            println!("swarm chat message: {}", m.message().to_ansi());
            if m.message().to_string() == "<py5> world" {
                for (name, world) in &swarm.instance_container.read().instances {
                    println!("world name: {name}");
                    if let Some(w) = world.upgrade() {
                        for chunk_pos in w.read().chunks.map.values() {
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
