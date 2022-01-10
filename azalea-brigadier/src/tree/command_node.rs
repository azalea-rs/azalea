use std::collections::HashMap;

use crate::{
    arguments::argument_type::{ArgumentResult, ArgumentType},
    builder::argument_builder::ArgumentBuilder,
    command::Command,
    context::{command_context::CommandContext, command_context_builder::CommandContextBuilder},
    exceptions::command_syntax_exception::CommandSyntaxException,
    redirect_modifier::RedirectModifier,
    string_reader::StringReader,
    suggestion::{suggestions::Suggestions, suggestions_builder::SuggestionsBuilder},
};

use super::{argument_command_node::ArgumentCommandNode, literal_command_node::LiteralCommandNode};

pub struct BaseCommandNode<S> {
    children: HashMap<String, dyn CommandNode<S>>,
    literals: HashMap<String, LiteralCommandNode<S>>,
    arguments: HashMap<String, ArgumentCommandNode<S, dyn ArgumentType<dyn ArgumentResult>>>,
    requirement: Option<dyn Fn(&S) -> bool>,
    redirect: Option<dyn CommandNode<S>>,
    modifier: Option<dyn RedirectModifier<S>>,
    forks: bool,
    command: Option<dyn Command<S>>,
}

impl<S> BaseCommandNode<S> {}

pub trait CommandNode<S> {
    fn name(&self) -> &str;
    fn usage_text(&self) -> &str;
    fn parse(
        &self,
        reader: StringReader,
        context_builder: CommandContextBuilder<S>,
    ) -> Result<(), CommandSyntaxException>;
    fn list_suggestions(
        &self,
        context: CommandContext<S>,
        builder: SuggestionsBuilder,
    ) -> Result<Suggestions, CommandSyntaxException>;
    fn is_valid_input(&self, input: &str) -> bool;
    fn create_builder<T>(&self) -> dyn ArgumentBuilder<S, T>;
    fn get_examples(&self) -> Vec<String>;
}
