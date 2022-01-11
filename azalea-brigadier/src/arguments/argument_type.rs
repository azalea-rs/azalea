use super::bool_argument_type::BoolArgumentType;
use crate::{
    context::command_context::CommandContext,
    exceptions::command_syntax_exception::CommandSyntaxException,
    string_reader::StringReader,
    suggestion::{suggestions::Suggestions, suggestions_builder::SuggestionsBuilder},
};

pub enum DefaultArguments {
    Bool(BoolArgumentType),
}

/*
define_arguments! {
    Entity(EntityArgumentType)
}

===

enum CustomArguments {
    Entity(EntityArgumentType)
}
enum BrigadierArguments {
    BuiltIn(DefaultArguments)
    Custom(CustomArguments)
}
*/

pub trait ArgumentType<T> {
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
