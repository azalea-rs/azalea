use std::time::Duration;

use azalea::{
    BlockPos, SprintDirection, WalkDirection,
    brigadier::prelude::*,
    pathfinder::goals::{BlockPosGoal, RadiusGoal, XZGoal},
    prelude::*,
};

use super::Ctx;
use crate::commands::Dispatcher;

pub fn register(commands: &mut Dispatcher) {
    commands.register(
        literal("goto")
            .executes(|ctx: &Ctx| {
                let source = ctx.source.lock();
                println!("got goto");
                // look for the sender
                let Some(entity) = source.entity() else {
                    source.reply("I can't see you!");
                    return Ok(0);
                };
                let position = entity.position()?;
                source.reply("ok");
                source
                    .bot
                    .start_goto(BlockPosGoal(BlockPos::from(position.up(0.5))));
                Ok(1)
            })
            .then(literal("xz").then(argument("x", integer()).then(
                argument("z", integer()).executes(|ctx: &Ctx| {
                    let source = ctx.source.lock();
                    let x = get_integer(ctx, "x").unwrap();
                    let z = get_integer(ctx, "z").unwrap();
                    println!("goto xz {x} {z}");
                    source.reply("ok");
                    source.bot.start_goto(XZGoal { x, z });
                    Ok(1)
                }),
            )))
            .then(literal("radius").then(argument("radius", float()).then(
                argument("x", integer()).then(argument("y", integer()).then(
                    argument("z", integer()).executes(|ctx: &Ctx| {
                        let source = ctx.source.lock();
                        let radius = get_float(ctx, "radius").unwrap();
                        let x = get_integer(ctx, "x").unwrap();
                        let y = get_integer(ctx, "y").unwrap();
                        let z = get_integer(ctx, "z").unwrap();
                        println!("goto radius {radius}, position: {x} {y} {z}");
                        source.reply("ok");
                        source.bot.start_goto(RadiusGoal {
                            pos: BlockPos::new(x, y, z).center(),
                            radius,
                        });
                        Ok(1)
                    }),
                )),
            )))
            .then(argument("x", integer()).then(argument("y", integer()).then(
                argument("z", integer()).executes(|ctx: &Ctx| {
                    let source = ctx.source.lock();
                    let x = get_integer(ctx, "x").unwrap();
                    let y = get_integer(ctx, "y").unwrap();
                    let z = get_integer(ctx, "z").unwrap();
                    println!("goto xyz {x} {y} {z}");
                    source.reply("ok");
                    source.bot.start_goto(BlockPosGoal(BlockPos::new(x, y, z)));
                    Ok(1)
                }),
            ))),
    );

    commands.register(literal("follow").executes(|ctx: &Ctx| {
        let source = ctx.source.lock();
        println!("got follow");
        // look for the sender
        let Some(entity) = source.entity() else {
            source.reply("I can't see you!");
            return Ok(0);
        };
        source.reply("ok");
        *source.state.following_entity.lock() = Some(entity);
        Ok(1)
    }));

    commands.register(literal("down").executes(|ctx: &Ctx| {
        let source = ctx.source.clone();
        let bot = source.lock().bot.clone();
        let position = BlockPos::from(bot.position()?);
        tokio::spawn(async move {
            source.lock().reply("mining...");
            bot.mine(position.down(1)).await;
            source.lock().reply("done");
        });
        Ok(1)
    }));

    commands.register(
        literal("look")
            .executes(|ctx: &Ctx| {
                // look for the sender
                let source = ctx.source.lock();
                let Some(entity) = source.entity() else {
                    source.reply("I can't see you!");
                    return Ok(0);
                };
                let eye_position = entity.eye_position()?;
                source.bot.look_at(eye_position);
                Ok(1)
            })
            .then(argument("x", integer()).then(argument("y", integer()).then(
                argument("z", integer()).executes(|ctx: &Ctx| {
                    let pos = BlockPos::new(
                        get_integer(ctx, "x").unwrap(),
                        get_integer(ctx, "y").unwrap(),
                        get_integer(ctx, "z").unwrap(),
                    );
                    println!("{pos:?}");
                    let source = ctx.source.lock();
                    source.bot.look_at(pos.center());
                    Ok(1)
                }),
            ))),
    );

    fn walk_command(ctx: &Ctx, direction: WalkDirection) -> eyre::Result<i32> {
        let mut seconds = get_float(ctx, "seconds").unwrap();
        let source = ctx.source.lock();
        let bot = source.bot.clone();

        if seconds < 0. {
            bot.walk(direction.opposite());
            seconds = -seconds;
        } else {
            bot.walk(direction);
        }

        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs_f32(seconds)).await;
            bot.walk(WalkDirection::None);
        });
        source.reply(format!("ok, walking {direction:?} for {seconds} seconds"));
        Ok(1)
    }

    commands.register(
        literal("walk").then(
            argument("seconds", float())
                .executes(|ctx: &Ctx| walk_command(ctx, WalkDirection::Forward)),
        ),
    );
    commands.register(literal("left").then(
        argument("seconds", float()).executes(|ctx: &Ctx| walk_command(ctx, WalkDirection::Left)),
    ));
    commands.register(literal("right").then(
        argument("seconds", float()).executes(|ctx: &Ctx| walk_command(ctx, WalkDirection::Right)),
    ));
    commands.register(
        literal("sprint").then(argument("seconds", float()).executes(|ctx: &Ctx| {
            let seconds = get_float(ctx, "seconds").unwrap();
            let source = ctx.source.lock();
            let bot = source.bot.clone();
            bot.sprint(SprintDirection::Forward);
            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_secs_f32(seconds)).await;
                bot.walk(WalkDirection::None);
            });
            source.reply(format!("ok, sprinting for {seconds} seconds"));
            Ok(1)
        })),
    );

    commands.register(literal("north").executes(|ctx: &Ctx| {
        let source = ctx.source.lock();
        source.bot.set_direction(180., 0.)?;
        source.reply("ok");
        Ok(1)
    }));
    commands.register(literal("south").executes(|ctx: &Ctx| {
        let source = ctx.source.lock();
        source.bot.set_direction(0., 0.)?;
        source.reply("ok");
        Ok(1)
    }));
    commands.register(literal("east").executes(|ctx: &Ctx| {
        let source = ctx.source.lock();
        source.bot.set_direction(-90., 0.)?;
        source.reply("ok");
        Ok(1)
    }));
    commands.register(literal("west").executes(|ctx: &Ctx| {
        let source = ctx.source.lock();
        source.bot.set_direction(90., 0.)?;
        source.reply("ok");
        Ok(1)
    }));
    commands.register(
        literal("jump")
            .executes(|ctx: &Ctx| {
                let source = ctx.source.lock();
                source.bot.jump();
                source.reply("ok");
                Ok(1)
            })
            .then(argument("enabled", bool()).executes(|ctx: &Ctx| {
                let jumping = get_bool(ctx, "enabled").unwrap();
                let source = ctx.source.lock();
                source.bot.set_jumping(jumping)?;
                Ok(1)
            })),
    );

    let sneak = |ctx: &Ctx| {
        let source = ctx.source.lock();
        source.bot.set_crouching(!source.bot.crouching())?;
        source.reply("ok");
        Ok(1)
    };
    let sneak_enabled = argument("enabled", bool()).executes(|ctx: &Ctx| {
        let sneaking = get_bool(ctx, "enabled").unwrap();
        let source = ctx.source.lock();
        source.bot.set_crouching(sneaking)?;
        Ok(1)
    });
    commands.register(literal("sneak").executes(sneak).then(sneak_enabled.clone()));
    commands.register(literal("crouch").executes(sneak).then(sneak_enabled));

    commands.register(literal("stop").executes(|ctx: &Ctx| {
        let source = ctx.source.lock();
        source.bot.stop_pathfinding();
        source.reply("ok");
        *source.state.following_entity.lock() = None;
        Ok(1)
    }));
    commands.register(literal("forcestop").executes(|ctx: &Ctx| {
        let source = ctx.source.lock();
        source.bot.force_stop_pathfinding();
        source.reply("ok");
        *source.state.following_entity.lock() = None;
        Ok(1)
    }));
}
