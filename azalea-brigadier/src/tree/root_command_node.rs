use std::fmt::{Display, Formatter};

use crate::{
    context::{command_context::CommandContext, command_context_builder::CommandContextBuilder},
    exceptions::{
        builtin_exceptions::BuiltInExceptions, command_syntax_exception::CommandSyntaxException,
    },
    string_reader::StringReader,
    suggestion::{suggestions::Suggestions, suggestions_builder::SuggestionsBuilder},
};

use super::command_node::{BaseCommandNode, CommandNode};

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct RootCommandNode<S> {
    // Since Rust doesn't have extending, we put the struct this is extending as the "base" field
    pub base: BaseCommandNode<S>,
}

impl<S> CommandNode<S> for RootCommandNode<S> {
    fn name(&self) -> &str {
        ""
    }

    fn parse(
        &self,
        reader: StringReader,
        context_builder: CommandContextBuilder<S>,
    ) -> Result<(), CommandSyntaxException> {
    }

    fn list_suggestions(
        &self,
        context: CommandContext<S>,
        builder: SuggestionsBuilder,
    ) -> Result<Suggestions, CommandSyntaxException> {
        Suggestions::empty()
    }

    fn is_valid_input(&self, input: &str) -> bool {
        false
    }

    fn usage_text(&self) -> &str {
        ""
    }

    fn create_builder(&self) -> () {
        panic!("Cannot convert root into a builder");
    }

    fn get_examples(&self) -> Vec<String> {
        vec![]
    }
}

impl Display for RootCommandNode<()> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<root>")
    }
}
