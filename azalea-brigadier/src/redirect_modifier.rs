use dyn_clonable::*;

use crate::{
    context::command_context::CommandContext,
    exceptions::command_syntax_exception::CommandSyntaxException,
};

#[clonable]
pub trait RedirectModifier<S, T>: Clone {
    fn apply(&self, context: CommandContext<S, T>) -> Result<Vec<S>, CommandSyntaxException>;
}
