use std::collections::HashMap;

use crate::{
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
    // private final Map<String, CommandNode<S>> children = new LinkedHashMap<>();
    // private final Map<String, LiteralCommandNode<S>> literals = new LinkedHashMap<>();
    // private final Map<String, ArgumentCommandNode<S, ?>> arguments = new LinkedHashMap<>();
    // private final Predicate<S> requirement;
    // private final CommandNode<S> redirect;
    // private final RedirectModifier<S> modifier;
    // private final boolean forks;
    // private Command<S> command;
    children: HashMap<String, dyn CommandNode<S>>,
    literals: HashMap<String, LiteralCommandNode<S>>,
    arguments: HashMap<String, ArgumentCommandNode<S, _>>,
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
