use crate::{
    context::command_context::CommandContext,
    exceptions::command_syntax_exception::CommandSyntaxException,
};

pub trait RedirectModifier<S> {
    fn apply(&self, context: CommandContext<S>) -> Result<Vec<S>, CommandSyntaxException>;
}
