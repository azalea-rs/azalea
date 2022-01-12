use crate::{
    context::command_context::CommandContext,
    exceptions::command_syntax_exception::CommandSyntaxException,
};
use dyn_clonable::*;

pub const SINGLE_SUCCESS: i32 = 1;

#[clonable]
pub trait Command<S, T>: Clone {
    fn run(&self, context: &mut CommandContext<S, T>) -> Result<i32, CommandSyntaxException>;
}
