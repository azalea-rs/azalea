use super::{argument_command_node::ArgumentCommandNode, literal_command_node::LiteralCommandNode};
use crate::{
    arguments::argument_type::{ArgumentType, Types},
    builder::argument_builder::ArgumentBuilder,
    command::Command,
    context::{command_context::CommandContext, command_context_builder::CommandContextBuilder},
    exceptions::command_syntax_exception::CommandSyntaxException,
    redirect_modifier::RedirectModifier,
    string_reader::StringReader,
    suggestion::{suggestions::Suggestions, suggestions_builder::SuggestionsBuilder},
};
use dyn_clonable::*;
use std::{collections::HashMap, fmt::Debug};

#[derive(Default)]
pub struct BaseCommandNode<'a, S, T>
where
    T: ArgumentType<dyn Types>,
{
    children: HashMap<String, &'a dyn CommandNode<S, T>>,
    literals: HashMap<String, LiteralCommandNode<'a, S, T>>,
    arguments: HashMap<String, ArgumentCommandNode<'a, S, T>>,
    requirement: Option<&'a dyn Fn(&S) -> bool>,
    redirect: Option<&'a dyn CommandNode<S, T>>,
    modifier: Option<&'a dyn RedirectModifier<S, T>>,
    forks: bool,
    command: Option<&'a dyn Command<S, T>>,
}

impl<S, T> BaseCommandNode<'_, S, T> where T: ArgumentType<dyn Types> {}

impl<S, T> Clone for BaseCommandNode<'_, S, T>
where
    T: ArgumentType<dyn Types>,
{
    fn clone(&self) -> Self {
        Self {
            children: self.children.clone(),
            literals: self.literals.clone(),
            arguments: self.arguments.clone(),
            requirement: self.requirement.clone(),
            redirect: self.redirect.clone(),
            modifier: self.modifier.clone(),
            forks: self.forks.clone(),
            command: self.command.clone(),
        }
    }
}

impl<S, T> Debug for BaseCommandNode<'_, S, T>
where
    T: ArgumentType<dyn Types>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BaseCommandNode")
            .field("children", &self.children)
            .field("literals", &self.literals)
            .field("arguments", &self.arguments)
            .field("requirement", &self.requirement)
            .field("redirect", &self.redirect)
            .field("modifier", &self.modifier)
            .field("forks", &self.forks)
            .field("command", &self.command)
            .finish()
    }
}

#[clonable]
pub trait CommandNode<S, T>: Clone
where
    T: ArgumentType<dyn Types>,
{
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
