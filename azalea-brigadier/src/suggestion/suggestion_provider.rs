use super::{Suggestions, SuggestionsBuilder};
use crate::context::CommandContext;

pub trait SuggestionProvider<S> {
    fn get_suggestions(
        &self,
        context: CommandContext<S>,
        builder: SuggestionsBuilder,
    ) -> Suggestions;
}
