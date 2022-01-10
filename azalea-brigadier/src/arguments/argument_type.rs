use crate::{
    context::command_context::CommandContext,
    exceptions::command_syntax_exception::CommandSyntaxException,
    string_reader::StringReader,
    suggestion::{suggestions::Suggestions, suggestions_builder::SuggestionsBuilder},
};

pub trait ArgumentResult {}

pub trait ArgumentType<T>
where
    T: ArgumentResult,
{
    // T parse(StringReader reader) throws CommandSyntaxException;

    // default <S> CompletableFuture<Suggestions> listSuggestions(final CommandContext<S> context, final SuggestionsBuilder builder) {
    //     return Suggestions.empty();
    // }

    // default Collection<String> getExamples() {
    //     return Collections.emptyList();
    // }

    fn parse(&self, reader: &mut StringReader) -> Result<T, CommandSyntaxException>;

    fn list_suggestions<S>(
        &self,
        context: &CommandContext<S>,
        builder: &mut SuggestionsBuilder,
    ) -> Result<Suggestions, CommandSyntaxException>;

    fn get_examples(&self) -> Vec<String>;
}
