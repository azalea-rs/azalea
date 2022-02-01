use super::argument_builder::BaseArgumentBuilder;
use crate::{
    arguments::argument_type::ArgumentType,
    command::Command,
    redirect_modifier::RedirectModifier,
    suggestion::suggestion_provider::SuggestionProvider,
    tree::{
        argument_command_node::ArgumentCommandNode,
        command_node::{BaseCommandNode, CommandNodeTrait},
        root_command_node::RootCommandNode,
    },
};
use std::any::Any;
use std::fmt::Debug;

pub struct RequiredArgumentBuilder<'a, S> {
    arguments: RootCommandNode<'a, S>,
    command: Option<Box<dyn Command<S>>>,
    requirement: Box<dyn Fn(&S) -> bool>,
    target: Option<Box<dyn CommandNodeTrait<S>>>,
    modifier: Option<Box<dyn RedirectModifier<S>>>,
    forks: bool,

    name: String,
    type_: Box<dyn ArgumentType<Into = dyn Any>>,
    suggestions_provider: Option<Box<dyn SuggestionProvider<S>>>,
}

impl<'a, S> RequiredArgumentBuilder<'a, S> {
    pub fn new(name: String, type_: Box<dyn ArgumentType<Into = dyn Any>>) -> Self {
        Self {
            name,
            type_: type_,
            suggestions_provider: None,
            arguments: RootCommandNode::new(),
            command: None,
            requirement: Box::new(|_| true),
            target: None,
            modifier: None,
            forks: false,
        }
    }

    pub fn argument(name: String, type_: Box<dyn ArgumentType<Into = dyn Any>>) -> Self {
        Self::new(name, type_)
    }

    pub fn suggests(mut self, provider: Box<dyn SuggestionProvider<S>>) -> Self {
        self.suggestions_provider = Some(provider);
        self
    }

    pub fn suggestions_provider(&self) -> Option<Box<dyn SuggestionProvider<S>>> {
        self.suggestions_provider
    }

    pub fn get_type(&self) -> Box<dyn ArgumentType<Into = dyn Any>> {
        self.type_
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    // final ArgumentCommandNode<S> result = new ArgumentCommandNode<>(getName(), getType(), getCommand(), getRequirement(), getRedirect(), getRedirectModifier(), isFork(), getSuggestionsProvider());

    // for (final CommandNode<S> argument : getArguments()) {
    // 	result.addChild(argument);
    // }

    // return result;
    pub fn build(self) -> ArgumentCommandNode<'a, S> {
        let result = ArgumentCommandNode {
            name: self.name,
            type_: self.type_,
            command: self.base.command(),
            requirement: self.base.requirement(),
            redirect: self.base.get_redirect(),
            modifier: self.base.get_redirect_modifier(),
            forks: self.base.forks,
            custom_suggestions: self.base.custom_suggestions,
            ..ArgumentCommandNode::default()
        };

        for argument in self.base.arguments() {
            result.add_child(argument);
        }

        result
    }
}
