use super::{
    argument_command_node::ArgumentCommandNode,
    command_node::{BaseCommandNode, CommandNodeTrait},
    literal_command_node::LiteralCommandNode,
};
use crate::{
    arguments::argument_type::ArgumentType,
    builder::argument_builder::ArgumentBuilder,
    context::{command_context::CommandContext, command_context_builder::CommandContextBuilder},
    exceptions::{
        builtin_exceptions::BuiltInExceptions, command_syntax_exception::CommandSyntaxException,
    },
    redirect_modifier::RedirectModifier,
    string_reader::StringReader,
    suggestion::{suggestions::Suggestions, suggestions_builder::SuggestionsBuilder},
};
use std::{
    any::Any,
    fmt::{Debug, Display, Formatter},
};

#[derive(Clone, Default, Debug)]
pub struct RootCommandNode<'a, S> {
    // Since Rust doesn't have extending, we put the struct this is extending as the "base" field
    pub base: BaseCommandNode<'a, S>,
}

impl<S> CommandNodeTrait<S> for RootCommandNode<'_, S> {
    fn name(&self) -> &str {
        ""
    }

    fn parse(
        &self,
        reader: &mut StringReader<'_>,
        context_builder: CommandContextBuilder<S>,
    ) -> Result<(), CommandSyntaxException> {
        Ok(())
    }

    fn list_suggestions(
        &self,
        context: CommandContext<S>,
        builder: &SuggestionsBuilder,
    ) -> Result<Suggestions, CommandSyntaxException> {
        Ok(Suggestions::default())
    }

    fn is_valid_input(&self, input: &str) -> bool {
        false
    }

    fn usage_text(&self) -> &str {
        ""
    }

    fn create_builder(&self) -> Box<dyn ArgumentBuilder<S>> {
        panic!("Cannot convert root into a builder");
    }

    fn get_examples(&self) -> Vec<String> {
        vec![]
    }
}

impl<S> Display for RootCommandNode<'_, S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<root>")
    }
}
