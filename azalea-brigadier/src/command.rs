use crate::{
    context::command_context::CommandContext,
    exceptions::command_syntax_exception::CommandSyntaxException,
};

pub const SINGLE_SUCCESS: i32 = 1;

pub trait Command<S> {
    fn run(&self, context: &mut CommandContext<S>) -> Result<i32, CommandSyntaxException>;
}
