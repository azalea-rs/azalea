use crate::{
    context::command_context::CommandContext,
    exceptions::command_syntax_exception::CommandSyntaxException,
};

pub trait SingleRedirectModifier<S, T> {
    fn apply(&self, context: CommandContext<S, T>) -> Result<S, CommandSyntaxException>;
}
