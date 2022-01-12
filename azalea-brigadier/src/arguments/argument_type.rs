use super::bool_argument_type::BoolArgumentType;
use crate::{
    context::command_context::CommandContext,
    exceptions::command_syntax_exception::CommandSyntaxException,
    string_reader::StringReader,
    suggestion::{suggestions::Suggestions, suggestions_builder::SuggestionsBuilder},
};
use dyn_clonable::*;

#[clonable]
// This should be applied to an Enum
pub trait Types: Clone {
    fn bool(value: bool) -> Self
    where
        Self: Sized;

    /// Get the less specific ArgumentType from this enum
    fn inner<T>(&self) -> Box<dyn ArgumentType<T>>
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

impl Types for BrigadierTypes {
    fn inner(&self) -> dyn ArgumentType<dyn Types> {
        match self {
            Bool(t) => t,
            Entity(t) => t
        }
    }
}
*/

#[clonable]
pub trait ArgumentType<T: ?Sized>: Clone
where
    T: Types,
{
    // T parse(StringReader reader) throws CommandSyntaxException;

    // default <S> CompletableFuture<Suggestions> listSuggestions(final CommandContext<S> context, final SuggestionsBuilder builder) {
    //     return Suggestions.empty();
    // }

    // default Collection<String> getExamples() {
    //     return Collections.emptyList();
    // }

    fn parse(&self, reader: &mut StringReader) -> Result<Box<T>, CommandSyntaxException>;

    fn list_suggestions<S>(
        &self,
        context: &CommandContext<S, T>,
        builder: &mut SuggestionsBuilder,
    ) -> Result<Suggestions, CommandSyntaxException>
    where
        Self: Sized,
        S: Sized,
        T: Sized;

    fn get_examples(&self) -> Vec<String>;
}
