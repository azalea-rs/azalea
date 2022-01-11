use std::collections::HashMap;

use crate::{
    arguments::argument_type::ArgumentType,
    builder::argument_builder::ArgumentBuilder,
    command::Command,
    context::{command_context::CommandContext, command_context_builder::CommandContextBuilder},
    exceptions::command_syntax_exception::CommandSyntaxException,
    redirect_modifier::RedirectModifier,
    string_reader::StringReader,
    suggestion::{suggestions::Suggestions, suggestions_builder::SuggestionsBuilder},
};

use super::{argument_command_node::ArgumentCommandNode, literal_command_node::LiteralCommandNode};

pub struct BaseCommandNode<'a, S, T> {
    children: HashMap<String, &'a dyn CommandNode<S, T>>,
    literals: HashMap<String, LiteralCommandNode<'a, S, T>>,
    arguments: HashMap<String, ArgumentCommandNode<'a, S, T>>,
    requirement: Option<&'a dyn Fn(&S) -> bool>,
    redirect: Option<&'a dyn CommandNode<S, T>>,
    modifier: Option<&'a dyn RedirectModifier<S, T>>,
    forks: bool,
    command: Option<&'a dyn Command<S, T>>,
}

impl<S, T> BaseCommandNode<'_, S, T> {}

pub trait CommandNode<S, T> {
    fn name(&self) -> &str;
    fn usage_text(&self) -> &str;
    fn parse(
        &self,
        reader: &mut StringReader,
        context_builder: CommandContextBuilder<S, T>,
    ) -> Result<(), CommandSyntaxException>;
    fn list_suggestions(
        &self,
        context: CommandContext<S, T>,
        builder: SuggestionsBuilder,
    ) -> Result<Suggestions, CommandSyntaxException>;
    fn is_valid_input(&self, input: &str) -> bool;
    fn create_builder(&self) -> dyn ArgumentBuilder<S, T>;
    fn get_examples(&self) -> Vec<String>;
}
