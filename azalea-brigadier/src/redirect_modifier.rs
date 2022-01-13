use dyn_clonable::*;

use crate::{
    context::command_context::CommandContext,
    exceptions::command_syntax_exception::CommandSyntaxException,
};

#[clonable]
pub trait RedirectModifier<S>: Clone {
    fn apply(&self, context: CommandContext<S>) -> Result<Vec<S>, CommandSyntaxException>;
}
