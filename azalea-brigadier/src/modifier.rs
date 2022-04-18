use std::rc::Rc;

use crate::{
    context::CommandContext, exceptions::command_syntax_exception::CommandSyntaxException,
};

pub type RedirectModifier<S> =
    dyn Fn(&CommandContext<S>) -> Result<Vec<Rc<S>>, CommandSyntaxException>;
