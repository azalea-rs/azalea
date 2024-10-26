use std::{any::Any, fmt::Debug, sync::Arc};

use super::argument_builder::{ArgumentBuilder, ArgumentBuilderType};
use crate::{
    arguments::ArgumentType,
    exceptions::CommandSyntaxException,
    string_reader::StringReader,
    suggestion::{Suggestions, SuggestionsBuilder},
};

/// An argument node type. The `T` type parameter is the type of the argument,
/// which can be anything.
#[derive(Clone)]
pub struct Argument {
    pub name: String,
    parser: Arc<dyn ArgumentType + Send + Sync>,
}
impl Argument {
    pub fn new(name: &str, parser: Arc<dyn ArgumentType + Send + Sync>) -> Self {
        Self {
            name: name.to_string(),
            parser,
        }
    }

    pub fn parse(&self, reader: &mut StringReader) -> Result<Arc<dyn Any>, CommandSyntaxException> {
        self.parser.parse(reader)
    }

    pub fn list_suggestions(&self, builder: SuggestionsBuilder) -> Suggestions {
        // TODO: custom suggestions
        // https://github.com/Mojang/brigadier/blob/master/src/main/java/com/mojang/brigadier/tree/ArgumentCommandNode.java#L71

        self.parser.list_suggestions(builder)
    }

    pub fn examples(&self) -> Vec<String> {
        self.parser.examples()
    }
}

impl From<Argument> for ArgumentBuilderType {
    fn from(argument: Argument) -> Self {
        Self::Argument(argument)
    }
}

impl Debug for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Argument")
            .field("name", &self.name)
            // .field("parser", &self.parser)
            .finish()
    }
}

/// Shortcut for creating a new argument builder node.
pub fn argument<S>(
    name: &str,
    parser: impl ArgumentType + Send + Sync + 'static,
) -> ArgumentBuilder<S> {
    ArgumentBuilder::new(Argument::new(name, Arc::new(parser)).into())
}
