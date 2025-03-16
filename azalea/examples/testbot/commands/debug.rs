//! Commands for debugging and getting the current state of the bot.

use std::{env, fs::File, io::Write, thread, time::Duration};

use azalea::{
    BlockPos,
    brigadier::prelude::*,
    chunks::ReceiveChunkEvent,
    entity::{LookDirection, Position},
    interact::HitResultComponent,
    packet::game,
    pathfinder::{ExecutingPath, Pathfinder},
    world::MinecraftEntityId,
};
use azalea_world::InstanceContainer;
use bevy_ecs::event::Events;
use parking_lot::Mutex;

use super::{CommandSource, Ctx};

pub fn register(commands: &mut CommandDispatcher<Mutex<CommandSource>>) {
    commands.register(literal("ping").executes(|ctx: &Ctx| {
        let source = ctx.source.lock();
        source.reply("pong!");
        1
    }));

    commands.register(literal("whereami").executes(|ctx: &Ctx| {
        let mut source = ctx.source.lock();
        let Some(entity) = source.entity() else {
            source.reply("You aren't in render distance!");
            return 0;
        };
        let position = source.bot.entity_component::<Position>(entity);
        source.reply(&format!(
            "You are at {}, {}, {}",
            position.x, position.y, position.z
        ));
        1
    }));

    commands.register(literal("entityid").executes(|ctx: &Ctx| {
        let mut source = ctx.source.lock();
        let Some(entity) = source.entity() else {
            source.reply("You aren't in render distance!");
            return 0;
        };
        let entity_id = source.bot.entity_component::<MinecraftEntityId>(entity);
        source.reply(&format!(
            "Your Minecraft ID is {} and your ECS id is {entity:?}",
            *entity_id
        ));
        1
    }));

    let whereareyou = |ctx: &Ctx| {
        let source = ctx.source.lock();
        let position = source.bot.position();
        source.reply(&format!(
            "I'm at {}, {}, {}",
            position.x, position.y, position.z
        ));
        1
    };
    commands.register(literal("whereareyou").executes(whereareyou));
    commands.register(literal("pos").executes(whereareyou));

    commands.register(literal("whoareyou").executes(|ctx: &Ctx| {
        let source = ctx.source.lock();
        source.reply(&format!(
            "I am {} ({})",
            source.bot.username(),
            source.bot.uuid()
        ));
        1
    }));

    commands.register(literal("getdirection").executes(|ctx: &Ctx| {
        let source = ctx.source.lock();
        let direction = source.bot.component::<LookDirection>();
        source.reply(&format!(
            "I'm looking at {}, {}",
            direction.y_rot, direction.x_rot
        ));
        1
    }));

    commands.register(literal("health").executes(|ctx: &Ctx| {
        let source = ctx.source.lock();

        let health = source.bot.health();
        source.reply(&format!("I have {health} health"));
        1
    }));

    commands.register(literal("lookingat").executes(|ctx: &Ctx| {
        let source = ctx.source.lock();

        let hit_result = *source.bot.component::<HitResultComponent>();

        if hit_result.miss {
            source.reply("I'm not looking at anything");
            return 1;
        }

        let block_pos = hit_result.block_pos;
        let block = source.bot.world().read().get_block_state(&block_pos);

        source.reply(&format!("I'm looking at {block:?} at {block_pos:?}"));

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
            let block = source.bot.world().read().get_block_state(&block_pos);
            source.reply(&format!("Block at {block_pos} is {block:?}"));
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
            let block = source.bot.world().read().get_fluid_state(&block_pos);
            source.reply(&format!("Fluid at {block_pos} is {block:?}"));
            1
        })),
    )));

    commands.register(literal("pathfinderstate").executes(|ctx: &Ctx| {
        let source = ctx.source.lock();
        let pathfinder = source.bot.get_component::<Pathfinder>();
        let Some(pathfinder) = pathfinder else {
            source.reply("I don't have the Pathfinder ocmponent");
            return 1;
        };
        source.reply(&format!(
            "pathfinder.is_calculating: {}",
            pathfinder.is_calculating
        ));

        let executing_path = source.bot.get_component::<ExecutingPath>();
        let Some(executing_path) = executing_path else {
            source.reply("I'm not executing a path");
            return 1;
        };
        source.reply(&format!(
            "is_path_partial: {}, path.len: {}, queued_path.len: {}",
            executing_path.is_path_partial,
            executing_path.path.len(),
            if let Some(queued) = executing_path.queued_path {
                queued.len().to_string()
            } else {
                "n/a".to_string()
            },
        ));
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

            let ecs = ecs.lock();



            let report_path = env::temp_dir().join("azalea-ecs-leak-report.txt");
            let mut report = File::create(&report_path).unwrap();

            for entity in ecs.iter_entities() {
                writeln!(report, "Entity: {}", entity.id()).unwrap();
                let archetype = entity.archetype();
                let component_count = archetype.component_count();

                let component_names = archetype
                    .components()
                    .map(|c| ecs.components().get_info(c).unwrap().name())
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
                let name = info.name();
                writeln!(report, "Resource: {name}").unwrap();
                // writeln!(report, "- Size: {} bytes",
                // info.layout().size()).unwrap();

                match name {
                    "azalea_world::container::InstanceContainer" => {
                        let instance_container = ecs.resource::<InstanceContainer>();

                        for (instance_name, instance) in &instance_container.instances {
                            writeln!(report, "- Name: {}", instance_name).unwrap();
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
                    "bevy_ecs::event::collections::Events<azalea_client::packet::game::ReceivePacketEvent>" => {
                        let events = ecs.resource::<Events<game::ReceivePacketEvent>>();
                        writeln!(report, "- Event count: {}", events.len()).unwrap();
                    }
                    "bevy_ecs::event::collections::Events<azalea_client::chunks::ReceiveChunkEvent>" => {
                        let events = ecs.resource::<Events<ReceiveChunkEvent>>();
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

        thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));

            std::process::exit(0);
        });

        1
    }));
}
