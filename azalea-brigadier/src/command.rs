use crate::{
    context::command_context::CommandContext,
    exceptions::command_syntax_exception::CommandSyntaxException,
};

pub const SINGLE_SUCCESS: i32 = 1;

pub trait Command<S, T> {
    fn run(&self, context: &mut CommandContext<S, T>) -> Result<i32, CommandSyntaxException>;
}
