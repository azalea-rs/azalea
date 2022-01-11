use crate::{
    context::command_context::CommandContext,
    exceptions::command_syntax_exception::CommandSyntaxException,
};

pub trait RedirectModifier<S, T> {
    fn apply(&self, context: CommandContext<S, T>) -> Result<Vec<S>, CommandSyntaxException>;
}
