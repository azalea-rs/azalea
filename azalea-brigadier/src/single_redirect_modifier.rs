use crate::{
    context::command_context::CommandContext,
    exceptions::command_syntax_exception::CommandSyntaxException,
};

pub trait SingleRedirectModifier<S> {
    fn apply(&self, context: CommandContext<S>) -> Result<S, CommandSyntaxException>;
}
