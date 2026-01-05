//! Commands for debugging and getting the current state of the bot.

use std::{env, fs::File, io::Write, thread, time::Duration};

use azalea::{
    BlockPos,
    brigadier::prelude::*,
    chunks::ReceiveChunkEvent,
    packet::game,
    pathfinder::{
        ExecutingPath, Pathfinder, custom_state::CustomPathfinderStateRef, mining::MiningCache,
        moves::PathfinderCtx, rel_block_pos::RelBlockPos, world::CachedWorld,
    },
};
use azalea_core::hit_result::HitResult;
use azalea_entity::{EntityKindComponent, metadata};
use azalea_inventory::components::MaxStackSize;
use azalea_world::InstanceContainer;
use bevy_app::AppExit;
use bevy_ecs::{message::Messages, query::With, world::EntityRef};
use parking_lot::Mutex;

use super::{CommandSource, Ctx};

pub fn register(commands: &mut CommandDispatcher<Mutex<CommandSource>>) {
    commands.register(literal("ping").executes(|ctx: &Ctx| {
        let source = ctx.source.lock();
        source.reply("pong!");
        1
    }));

    commands.register(literal("disconnect").executes(|ctx: &Ctx| {
        let source = ctx.source.lock();
        source.bot.disconnect();
        1
    }));

    commands.register(literal("whereami").executes(|ctx: &Ctx| {
        let source = ctx.source.lock();
        let Some(entity) = source.entity() else {
            source.reply("You aren't in render distance!");
            return 0;
        };
        let position = entity.position();
        source.reply(format!(
            "You are at {}, {}, {}",
            position.x, position.y, position.z
        ));
        1
    }));

    commands.register(literal("entityid").executes(|ctx: &Ctx| {
        let source = ctx.source.lock();
        let Some(entity) = source.entity() else {
            source.reply("You aren't in render distance!");
            return 0;
        };
        let entity_id = entity.minecraft_id();
        source.reply(format!(
            "Your Minecraft ID is {} and your ECS ID is {entity:?}",
            *entity_id
        ));
        1
    }));

    let whereareyou = |ctx: &Ctx| {
        let source = ctx.source.lock();
        let position = source.bot.position();
        source.reply(format!(
            "I'm at {}, {}, {}",
            position.x, position.y, position.z
        ));
        1
    };
    commands.register(literal("whereareyou").executes(whereareyou));
    commands.register(literal("pos").executes(whereareyou));

    commands.register(literal("whoareyou").executes(|ctx: &Ctx| {
        let source = ctx.source.lock();
        source.reply(format!(
            "I am {} ({}, {})",
            source.bot.username(),
            source.bot.uuid(),
            source.bot.entity
        ));
        1
    }));

    commands.register(literal("getdirection").executes(|ctx: &Ctx| {
        let source = ctx.source.lock();
        let direction = source.bot.direction();
        source.reply(format!(
            "I'm looking at {}, {}",
            direction.y_rot(),
            direction.x_rot()
        ));
        1
    }));

    commands.register(literal("health").executes(|ctx: &Ctx| {
        let source = ctx.source.lock();

        let health = source.bot.health();
        source.reply(format!("I have {health} health"));
        1
    }));

    commands.register(literal("lookingat").executes(|ctx: &Ctx| {
        let source = ctx.source.lock();

        let hit_result = source.bot.hit_result();

        match &hit_result {
            HitResult::Block(r) => {
                if r.miss {
                    source.reply("I'm not looking at anything");
                    return 0;
                }
                let block_pos = r.block_pos;
                let block = source.bot.world().read().get_block_state(block_pos);
                source.reply(format!("I'm looking at {block:?} at {block_pos:?}"));
            }
            HitResult::Entity(r) => {
                let entity_kind = **source.bot.entity_component::<EntityKindComponent>(r.entity);
                source.reply(format!(
                    "I'm looking at {entity_kind} ({:?}) at {}",
                    r.entity, r.location
                ));
            }
        }

        1
    }));

    commands.register(literal("getblock").then(argument("x", integer()).then(
        argument("y", integer()).then(argument("z", integer()).executes(|ctx: &Ctx| {
            let source = ctx.source.lock();
            let x = get_integer(ctx, "x").unwrap();
            let y = get_integer(ctx, "y").unwrap();
            let z = get_integer(ctx, "z").unwrap();
            println!("getblock xyz {x} {y} {z}");
            let block_pos = BlockPos::new(x, y, z);
            let block = source.bot.world().read().get_block_state(block_pos);
            source.reply(format!("BlockKind at {block_pos} is {block:?}"));
            1
        })),
    )));
    commands.register(literal("getfluid").then(argument("x", integer()).then(
        argument("y", integer()).then(argument("z", integer()).executes(|ctx: &Ctx| {
            let source = ctx.source.lock();
            let x = get_integer(ctx, "x").unwrap();
            let y = get_integer(ctx, "y").unwrap();
            let z = get_integer(ctx, "z").unwrap();
            println!("getfluid xyz {x} {y} {z}");
            let block_pos = BlockPos::new(x, y, z);
            let block = source.bot.world().read().get_fluid_state(block_pos);
            source.reply(format!("Fluid at {block_pos} is {block:?}"));
            1
        })),
    )));

    commands.register(literal("pathfinderstate").executes(|ctx: &Ctx| {
        let source = ctx.source.lock();
        let pathfinder = source.bot.get_component::<Pathfinder>();
        let Some(pathfinder) = pathfinder else {
            source.reply("I don't have the Pathfinder component");
            return 1;
        };
        source.reply(format!(
            "pathfinder.is_calculating: {}",
            pathfinder.is_calculating
        ));

        let executing_path = source.bot.get_component::<ExecutingPath>();
        let Some(executing_path) = executing_path else {
            source.reply("I'm not executing a path");
            return 1;
        };
        source.reply(format!(
            "is_path_partial: {}, path.len: {}, queued_path.len: {}",
            executing_path.is_path_partial,
            executing_path.path.len(),
            if let Some(queued) = &executing_path.queued_path {
                queued.len().to_string()
            } else {
                "n/a".to_owned()
            },
        ));
        1
    }));
    commands.register(literal("pathfindermoves").executes(|ctx: &Ctx| {
        let source = ctx.source.lock();

        let Some(entity) = source.entity() else {
            source.reply("You aren't in render distance!");
            return 0;
        };
        let position = entity.position();
        let position = BlockPos::from(position);

        let mut edges = Vec::new();
        let cached_world = CachedWorld::new(source.bot.world(), position);
        let mining_cache = MiningCache::new(None);
        let custom_state = CustomPathfinderStateRef::default();

        azalea::pathfinder::moves::default_move(
            &mut PathfinderCtx {
                edges: &mut edges,
                world: &cached_world,
                mining_cache: &mining_cache,
                custom_state: &custom_state,
            },
            RelBlockPos::from_origin(position, position),
        );

        if edges.is_empty() {
            source.reply("No possible moves.");
        } else {
            source.reply("Moves:");
            for (i, edge) in edges.iter().enumerate() {
                source.reply(format!("{}) {edge:?}", i + 1));
            }
        }

        1
    }));

    commands.register(literal("startuseitem").executes(|ctx: &Ctx| {
        let source = ctx.source.lock();
        source.bot.start_use_item();
        source.reply("Ok!");
        1
    }));
    commands.register(literal("maxstacksize").executes(|ctx: &Ctx| {
        let source = ctx.source.lock();
        let max_stack_size = source
            .bot
            .get_held_item()
            .get_component::<MaxStackSize>()
            .map_or(-1, |s| s.count);
        source.reply(format!("{max_stack_size}"));
        1
    }));

    commands.register(literal("dimensions").executes(|ctx: &Ctx| {
        let source = ctx.source.lock();
        let bot_dimensions = source.bot.dimensions();
        source.reply(format!("{bot_dimensions:?}"));
        1
    }));

    commands.register(literal("players").executes(|ctx: &Ctx| {
        let source = ctx.source.lock();
        let player_entities = source
            .bot
            .nearest_entities_by::<(), With<metadata::Player>>(|_: ()| true);
        let tab_list = source.bot.tab_list();
        for player_entity in player_entities {
            let uuid = player_entity.uuid();
            source.reply(format!(
                "{} - {} ({:?})",
                player_entity.id(),
                tab_list.get(&uuid).map_or("?", |p| p.profile.name.as_str()),
                uuid
            ));
        }
        1
    }));

    commands.register(literal("enchants").executes(|ctx: &Ctx| {
        let source = ctx.source.lock();
        source.bot.with_registry_holder(|r| {
            let enchants = &r.enchantment;
            println!("enchants: {enchants:?}");
        });
        1
    }));

    commands.register(literal("attributes").executes(|ctx: &Ctx| {
        let source = ctx.source.lock();
        let attributes = source.bot.attributes();
        println!("attributes: {attributes:?}");
        1
    }));

    commands.register(literal("debugecsleak").executes(|ctx: &Ctx| {
        let source = ctx.source.lock();

        source.reply("Ok!");



        source.bot.disconnect();

        let ecs = source.bot.ecs.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            // dump the ecs

            let mut ecs = ecs.write();

            let report_path = env::temp_dir().join("azalea-ecs-leak-report.txt");
            let mut report = File::create(&report_path).unwrap();

            let mut query = ecs.query::<EntityRef>();
            for entity in query.iter(& ecs) {
                writeln!(report, "Entity: {}", entity.id()).unwrap();
                let archetype = entity.archetype();
                let component_count = archetype.component_count();

                let component_names = archetype
                    .components()
                    .iter()
                    .map(|c| ecs.components().get_info(*c).unwrap().name().to_string())
                    .collect::<Vec<_>>();
                writeln!(
                    report,
                    "- {component_count} components: {}",
                    component_names.join(", ")
                )
                .unwrap();
            }

            writeln!(report).unwrap();


            for (info, _) in ecs.iter_resources() {
                let name = info.name().to_string();
                writeln!(report, "Resource: {name}").unwrap();
                // writeln!(report, "- Size: {} bytes",
                // info.layout().size()).unwrap();

                match name.as_ref() {
                    "azalea_world::container::InstanceContainer" => {
                        let instance_container = ecs.resource::<InstanceContainer>();

                        for (instance_name, instance) in &instance_container.instances {
                            writeln!(report, "- Name: {instance_name}").unwrap();
                            writeln!(report, "- Reference count: {}", instance.strong_count())
                                .unwrap();
                            if let Some(instance) = instance.upgrade() {
                                let instance = instance.read();
                                let strong_chunks = instance
                                    .chunks
                                    .map
                                    .iter()
                                    .filter(|(_, v)| v.strong_count() > 0)
                                    .count();
                                writeln!(
                                    report,
                                    "- Chunks: {} strongly referenced, {} in map",
                                    strong_chunks,
                                    instance.chunks.map.len()
                                )
                                .unwrap();
                                writeln!(
                                    report,
                                    "- Entities: {}",
                                    instance.entities_by_chunk.len()
                                )
                                .unwrap();
                            }
                        }
                    }
                    "bevy_ecs::message::Messages<azalea_client::packet::game::ReceivePacketEvent>" => {
                        let events = ecs.resource::<Messages<game::ReceiveGamePacketEvent>>();
                        writeln!(report, "- Event count: {}", events.len()).unwrap();
                    }
                    "bevy_ecs::message::Messages<azalea_client::chunks::ReceiveChunkEvent>" => {
                        let events = ecs.resource::<Messages<ReceiveChunkEvent>>();
                        writeln!(report, "- Event count: {}", events.len()).unwrap();
                    }

                    _ => {}
                }
            }

            println!("\x1b[1mWrote report to {}\x1b[m", report_path.display());
        });

        1
    }));

    commands.register(literal("exit").executes(|ctx: &Ctx| {
        let source = ctx.source.lock();
        source.reply("bye!");

        source.bot.disconnect();

        let source = ctx.source.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));

            source
                .lock()
                .bot
                .ecs
                .write()
                .write_message(AppExit::Success);
        });

        1
    }));
}
