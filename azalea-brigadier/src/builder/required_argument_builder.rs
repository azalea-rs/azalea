use crate::{
    arguments::argument_type::{ArgumentType, Types},
    suggestion::suggestion_provider::SuggestionProvider,
    tree::{argument_command_node::ArgumentCommandNode, command_node::BaseCommandNode},
};

use super::argument_builder::BaseArgumentBuilder;

pub struct RequiredArgumentBuilder<'a, S, T>
where
    T: ArgumentType<dyn Types>,
{
    // private final String name;
    // private final ArgumentType<T> type;
    // private SuggestionProvider<S> suggestionsProvider = null;
    name: String,
    type_: &'a T,
    suggestions_provider: Option<&'a dyn SuggestionProvider<S, T>>,

    pub base: BaseArgumentBuilder<'a, S, T>,
}

impl<'a, S, T> RequiredArgumentBuilder<'a, S, T>
where
    T: ArgumentType<dyn Types>,
{
    pub fn new(name: String, type_: T) -> Self {
        Self {
            name,
            type_: &type_,
            suggestions_provider: None,
            base: BaseArgumentBuilder::new(name, type_),
        }
    }

    pub fn argument(name: String, type_: T) -> Self {
        Self::new(name, type_)
    }

    pub fn suggests(mut self, provider: &dyn SuggestionProvider<S, T>) -> Self {
        self.suggestions_provider = Some(provider);
        self
    }

    pub fn suggestions_provider(&self) -> Option<&dyn SuggestionProvider<S, T>> {
        self.suggestions_provider.as_ref()
    }

    pub fn get_type(&self) -> &T {
        self.type_
    }

    pub fn name(&self) -> &str {
        self.name
    }

    // final ArgumentCommandNode<S, T> result = new ArgumentCommandNode<>(getName(), getType(), getCommand(), getRequirement(), getRedirect(), getRedirectModifier(), isFork(), getSuggestionsProvider());

    // for (final CommandNode<S> argument : getArguments()) {
    // 	result.addChild(argument);
    // }

    // return result;
    pub fn build(self) -> ArgumentCommandNode<'a, S, T> {
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
