use azalea::brigadier::prelude::*;
use parking_lot::Mutex;

use super::{CommandSource, Ctx};
use crate::State;

pub fn register(commands: &mut CommandDispatcher<Mutex<CommandSource>>) {
    commands.register(
        literal("killaura").then(argument("enabled", bool()).executes(|ctx: &Ctx| {
            let enabled = get_bool(ctx, "enabled").unwrap();
            let source = ctx.source.lock();
            let bot = source.bot.clone();
            bot.query_self::<&mut State, _>(|mut state| state.killaura = enabled);
            source.reply(if enabled {
                "Enabled killaura"
            } else {
                "Disabled killaura"
            });
            1
        })),
    );
}
