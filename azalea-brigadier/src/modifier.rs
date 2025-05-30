use std::sync::Arc;

use crate::{context::CommandContext, errors::CommandSyntaxError};

pub type RedirectModifier<S> =
    dyn Fn(&CommandContext<S>) -> Result<Vec<Arc<S>>, CommandSyntaxError> + Send + Sync;
