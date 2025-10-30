use std::{
    any::Any,
    fmt::{self, Debug},
    sync::Arc,
};

use super::argument_builder::{ArgumentBuilder, ArgumentBuilderType};
use crate::{
    arguments::ArgumentType,
    context::CommandContext,
    errors::CommandSyntaxError,
    string_reader::StringReader,
    suggestion::{SuggestionProvider, Suggestions, SuggestionsBuilder},
};

/// An argument node type.
///
/// The `T` type parameter is the type of the argument, which can be anything.
pub struct Argument<S> {
    pub name: String,
    parser: Arc<dyn ArgumentType + Send + Sync>,
    custom_suggestions: Option<Arc<dyn SuggestionProvider<S> + Send + Sync>>,
}
impl<S> Argument<S> {
    pub fn new(
        name: &str,
        parser: Arc<dyn ArgumentType + Send + Sync>,
        custom_suggestions: Option<Arc<dyn SuggestionProvider<S> + Send + Sync>>,
    ) -> Self {
        Self {
            name: name.to_string(),
            parser,
            custom_suggestions,
        }
    }

    pub fn parse(&self, reader: &mut StringReader) -> Result<Arc<dyn Any>, CommandSyntaxError> {
        self.parser.parse(reader)
    }

    pub fn list_suggestions(
        &self,
        context: CommandContext<S>,
        builder: SuggestionsBuilder,
    ) -> Suggestions {
        if let Some(s) = &self.custom_suggestions {
            s.get_suggestions(context, builder)
        } else {
            self.parser.list_suggestions(builder)
        }
    }

    pub fn examples(&self) -> Vec<String> {
        self.parser.examples()
    }
}

impl<S> From<Argument<S>> for ArgumentBuilderType<S> {
    fn from(argument: Argument<S>) -> Self {
        Self::Argument(argument)
    }
}

impl<S> Debug for Argument<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    ArgumentBuilder::new(Argument::new(name, Arc::new(parser), None).into())
}

impl<S> Clone for Argument<S> {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            parser: self.parser.clone(),
            custom_suggestions: self.custom_suggestions.clone(),
        }
    }
}
