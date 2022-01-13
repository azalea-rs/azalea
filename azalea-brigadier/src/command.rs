use crate::{
    context::command_context::CommandContext,
    exceptions::command_syntax_exception::CommandSyntaxException,
};
use dyn_clonable::*;

pub const SINGLE_SUCCESS: i32 = 1;

#[clonable]
pub trait Command<S>: Clone {
    fn run(&self, context: &mut CommandContext<S>) -> Result<i32, CommandSyntaxException>;
}
