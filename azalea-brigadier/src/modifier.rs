use std::{any::Any, rc::Rc};

use crate::{
    context::CommandContext, exceptions::command_syntax_exception::CommandSyntaxException,
};

pub trait RedirectModifier<S: Any + Clone> {
    fn apply(&self, context: &CommandContext<S>) -> Result<Vec<Rc<S>>, CommandSyntaxException>;
}
