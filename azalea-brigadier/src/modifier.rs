use std::sync::Arc;

use crate::{context::CommandContext, exceptions::CommandSyntaxException};

pub type RedirectModifier<S> =
    dyn Fn(&CommandContext<S>) -> Result<Vec<Arc<S>>, CommandSyntaxException> + Send + Sync;
