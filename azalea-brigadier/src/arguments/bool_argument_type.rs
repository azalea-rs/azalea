use crate::{
    context::command_context::CommandContext,
    exceptions::command_syntax_exception::CommandSyntaxException,
    string_reader::StringReader,
    suggestion::{suggestions::Suggestions, suggestions_builder::SuggestionsBuilder},
};

use super::argument_type::{ArgumentType, Types};

#[derive(Clone)]
pub struct BoolArgumentType {}

impl<T> ArgumentType<T> for BoolArgumentType
where
    T: Types,
{
    fn parse(&self, reader: &mut StringReader) -> Result<T, CommandSyntaxException> {
        Ok(T::bool(reader.read_boolean()?))
    }

    fn list_suggestions<S>(
        &self,
        context: &CommandContext<S, T>,
        builder: &mut SuggestionsBuilder,
    ) -> Result<Suggestions, CommandSyntaxException>
    where
        S: Sized,
        T: Sized,
    {
        // if ("true".startsWith(builder.getRemainingLowerCase())) {
        //     builder.suggest("true");
        // }
        // if ("false".startsWith(builder.getRemainingLowerCase())) {
        //     builder.suggest("false");
        // }
        // return builder.buildFuture();
        if "true".starts_with(builder.remaining_lowercase()) {
            builder.suggest("true");
        }
        if "false".starts_with(builder.remaining_lowercase()) {
            builder.suggest("false");
        }
        Ok(builder.build())
    }

    fn get_examples(&self) -> Vec<String> {
        vec![]
    }
}

impl BoolArgumentType {
    const EXAMPLES: &'static [&'static str] = &["true", "false"];

    fn bool() -> Self {
        Self {}
    }

    fn get_bool<S, T>(context: CommandContext<S, T>, name: String) {
        context.get_argument::<bool>(name)
    }
}
