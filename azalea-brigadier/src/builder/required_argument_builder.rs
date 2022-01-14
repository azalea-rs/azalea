use crate::{
    arguments::argument_type::ArgumentType,
    suggestion::suggestion_provider::SuggestionProvider,
    tree::{argument_command_node::ArgumentCommandNode, command_node::BaseCommandNode},
};
use std::any::Any;

use super::argument_builder::BaseArgumentBuilder;

pub struct RequiredArgumentBuilder<'a, S> {
    // private final String name;
    // private final ArgumentType<T> type;
    // private SuggestionProvider<S> suggestionsProvider = null;
    name: String,
    type_: Box<dyn ArgumentType<Into = dyn Any>>,
    suggestions_provider: Option<&'a dyn SuggestionProvider<S>>,

    pub base: BaseArgumentBuilder<'a, S>,
}

impl<'a, S> RequiredArgumentBuilder<'a, S> {
    pub fn new(name: String, type_: Box<dyn ArgumentType<Into = dyn Any>>) -> Self {
        Self {
            name,
            type_: type_,
            suggestions_provider: None,
            base: BaseArgumentBuilder::new(name, type_),
        }
    }

    pub fn argument(name: String, type_: dyn ArgumentType<Into = dyn Any>) -> Self {
        Self::new(name, type_)
    }

    pub fn suggests(mut self, provider: &dyn SuggestionProvider<S>) -> Self {
        self.suggestions_provider = Some(provider);
        self
    }

    pub fn suggestions_provider(&self) -> Option<&dyn SuggestionProvider<S>> {
        self.suggestions_provider.as_ref()
    }

    pub fn get_type(&self) -> &dyn ArgumentType<Into = dyn Any> {
        self.type_
    }

    pub fn name(&self) -> &str {
        self.name
    }

    // final ArgumentCommandNode<S> result = new ArgumentCommandNode<>(getName(), getType(), getCommand(), getRequirement(), getRedirect(), getRedirectModifier(), isFork(), getSuggestionsProvider());

    // for (final CommandNode<S> argument : getArguments()) {
    // 	result.addChild(argument);
    // }

    // return result;
    pub fn build(self) -> ArgumentCommandNode<'a, S> {
        let result = ArgumentCommandNode {
            name: self.name,
            type_: &self.type_,
            base: BaseCommandNode {
                command: self.base.command,
                requirement: self.base.requirement,
                redirect: self.base.redirect,
                modifier: self.base.modifier,
                forks: self.base.forks,
                ..BaseCommandNode::default()
            },
            custom_suggestions: self.base.custom_suggestions,
        };

        for argument in self.base.arguments {
            result.add_child(argument);
        }

        result
    }
}
