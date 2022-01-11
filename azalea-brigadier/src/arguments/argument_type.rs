use super::bool_argument_type::BoolArgumentType;
use crate::{
    context::command_context::CommandContext,
    exceptions::command_syntax_exception::CommandSyntaxException,
    string_reader::StringReader,
    suggestion::{suggestions::Suggestions, suggestions_builder::SuggestionsBuilder},
};

pub trait Types {
    fn bool(value: bool) -> Self
    where
        Self: Sized;
}

/*
#[derive(Types)]
enum BrigadierTypes {
    Entity(EntityArgumentType)
}

===

enum BrigadierTypes {
    Bool(BoolArgumentType)

    Entity(EntityArgumentType)
}
*/

pub trait ArgumentType<T>
where
    Self: Sized,
    T: Types + ?Sized,
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
        context: &CommandContext<S, T>,
        builder: &mut SuggestionsBuilder,
    ) -> Result<Suggestions, CommandSyntaxException>
    where
        S: Sized,
        T: Sized;

    fn get_examples(&self) -> Vec<String>;
}
