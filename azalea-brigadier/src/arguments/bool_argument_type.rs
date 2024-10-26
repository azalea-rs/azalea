use std::{any::Any, sync::Arc};

use super::ArgumentType;
use crate::{
    context::CommandContext,
    exceptions::CommandSyntaxException,
    string_reader::StringReader,
    suggestion::{Suggestions, SuggestionsBuilder},
};

#[derive(Default)]
struct Boolean;

impl ArgumentType for Boolean {
    fn parse(&self, reader: &mut StringReader) -> Result<Arc<dyn Any>, CommandSyntaxException> {
        Ok(Arc::new(reader.read_boolean()?))
    }

    fn list_suggestions(&self, mut builder: SuggestionsBuilder) -> Suggestions {
        if "true".starts_with(builder.remaining_lowercase()) {
            builder = builder.suggest("true");
        }
        if "false".starts_with(builder.remaining_lowercase()) {
            builder = builder.suggest("false");
        }
        builder.build()
    }

    fn examples(&self) -> Vec<String> {
        vec!["true".to_string(), "false".to_string()]
    }
}

pub fn bool() -> impl ArgumentType {
    Boolean
}
pub fn get_bool<S>(context: &CommandContext<S>, name: &str) -> Option<bool> {
    context
        .argument(name)
        .expect("argument with name not found")
        .downcast_ref::<bool>()
        .cloned()
}
