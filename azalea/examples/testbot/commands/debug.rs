//! Commands for debugging and getting the current state of the bot.

use azalea::{
    brigadier::prelude::*,
    entity::{LookDirection, Position},
    interact::HitResultComponent,
    pathfinder::{ExecutingPath, Pathfinder},
    world::MinecraftEntityId,
    BlockPos,
};
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
            source.reply(&format!("Block at {block_pos:?} is {block:?}"));
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
}
