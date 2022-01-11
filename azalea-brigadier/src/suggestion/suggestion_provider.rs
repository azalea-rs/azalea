use crate::{
    context::command_context::CommandContext,
    exceptions::command_syntax_exception::CommandSyntaxException,
};

use super::{suggestions::Suggestions, suggestions_builder::SuggestionsBuilder};

pub trait SuggestionProvider<S, T> {
    fn suggestions(
        &self,
        context: &CommandContext<S, T>,
        builder: &SuggestionsBuilder,
    ) -> Result<Suggestions, CommandSyntaxException>;
}
