use azalea::brigadier::prelude::*;
use parking_lot::Mutex;

use super::{CommandSource, Ctx};

pub fn register(commands: &mut CommandDispatcher<Mutex<CommandSource>>) {
    commands.register(
        literal("killaura").then(argument("enabled", bool()).executes(|ctx: &Ctx| {
            let enabled = get_bool(ctx, "enabled").unwrap();
            let source = ctx.source.lock();
            let bot = source.bot.clone();
            {
                let mut ecs = bot.ecs.lock();
                let mut entity = ecs.entity_mut(bot.entity);
                let mut state = entity.get_mut::<crate::State>().unwrap();
                state.killaura = enabled
            }
            source.reply(if enabled {
                "Enabled killaura"
            } else {
                "Disabled killaura"
            });
            1
        })),
    );
}
