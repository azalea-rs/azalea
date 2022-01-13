use super::bool_argument_type::BoolArgumentType;
use crate::{
    context::command_context::CommandContext,
    exceptions::command_syntax_exception::CommandSyntaxException,
    string_reader::StringReader,
    suggestion::{suggestions::Suggestions, suggestions_builder::SuggestionsBuilder},
};
use dyn_clonable::*;

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
pub trait ArgumentType: Clone {
    type Into;
    // T parse(StringReader reader) throws CommandSyntaxException;

    // default <S> CompletableFuture<Suggestions> listSuggestions(final CommandContext<S> context, final SuggestionsBuilder builder) {
    //     return Suggestions.empty();
    // }

    // default Collection<String> getExamples() {
    //     return Collections.emptyList();
    // }

    fn parse(&self, reader: &mut StringReader) -> Result<Self::Into, CommandSyntaxException>;

    fn list_suggestions<S>(
        &self,
        context: &CommandContext<S>,
        builder: &mut SuggestionsBuilder,
    ) -> Result<Suggestions, CommandSyntaxException>
    where
        Self: Sized,
        S: Sized;

    fn get_examples(&self) -> Vec<String>;
}
