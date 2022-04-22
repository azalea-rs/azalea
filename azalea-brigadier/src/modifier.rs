use std::rc::Rc;

use crate::{context::CommandContext, exceptions::CommandSyntaxException};

pub type RedirectModifier<S> =
    dyn Fn(&CommandContext<S>) -> Result<Vec<Rc<S>>, CommandSyntaxException>;
