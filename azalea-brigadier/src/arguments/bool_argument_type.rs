use crate::{
    context::command_context::CommandContext,
    exceptions::command_syntax_exception::CommandSyntaxException,
    string_reader::StringReader,
    suggestion::{suggestions::Suggestions, suggestions_builder::SuggestionsBuilder},
};

use super::argument_type::ArgumentType;

#[derive(Clone)]
pub struct BoolArgumentType {}

impl ArgumentType for BoolArgumentType {
    type Into = bool;

    fn parse(&self, reader: &mut StringReader) -> Result<Self::Into, CommandSyntaxException> {
        Ok(reader.read_boolean()?)
    }

    fn list_suggestions<S>(
        &self,
        context: &CommandContext<S>,
        builder: &mut SuggestionsBuilder,
    ) -> Result<Suggestions, CommandSyntaxException>
    where
        S: Sized,
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

    fn get_bool<S>(context: CommandContext<S>, name: String) {
        context.get_argument::<bool>(name)
    }
}
