use crate::{
    arguments::argument_type::ArgumentType,
    suggestion::suggestion_provider::SuggestionProvider,
    tree::{argument_command_node::ArgumentCommandNode, command_node::BaseCommandNode},
};

use super::argument_builder::BaseArgumentBuilder;

// private RequiredArgumentBuilder(final String name, final ArgumentType<T> type) {
// 	this.name = name;
// 	this.type = type;
// }

// public static <S, T> RequiredArgumentBuilder<S, T> argument(final String name, final ArgumentType<T> type) {
// 	return new RequiredArgumentBuilder<>(name, type);
// }

// public RequiredArgumentBuilder<S, T> suggests(final SuggestionProvider<S> provider) {
// 	this.suggestionsProvider = provider;
// 	return getThis();
// }

// public SuggestionProvider<S> getSuggestionsProvider() {
// 	return suggestionsProvider;
// }

// @Override
// protected RequiredArgumentBuilder<S, T> getThis() {
// 	return this;
// }

// public ArgumentType<T> getType() {
// 	return type;
// }

// public String getName() {
// 	return name;
// }

// public ArgumentCommandNode<S, T> build() {
// 	final ArgumentCommandNode<S, T> result = new ArgumentCommandNode<>(getName(), getType(), getCommand(), getRequirement(), getRedirect(), getRedirectModifier(), isFork(), getSuggestionsProvider());

// 	for (final CommandNode<S> argument : getArguments()) {
// 		result.addChild(argument);
// 	}

// 	return result;
// }

pub struct RequiredArgumentBuilder<S, T> {
    // private final String name;
    // private final ArgumentType<T> type;
    // private SuggestionProvider<S> suggestionsProvider = null;
    name: String,
    type_: dyn ArgumentType<T>,
    suggestions_provider: Option<dyn SuggestionProvider<S>>,

    pub base: BaseArgumentBuilder<S, T>,
}

impl<S, T> RequiredArgumentBuilder<S, T> {
    pub fn new(name: String, type_: dyn ArgumentType<T>) -> Self {
        Self {
            name,
            type_,
            suggestions_provider: None,
            base: BaseArgumentBuilder::new(name, type_),
        }
    }

    pub fn argument(name: String, type_: dyn ArgumentType<T>) -> Self {
        Self::new(name, type_)
    }

    pub fn suggests(mut self, provider: dyn SuggestionProvider<S>) -> Self {
        self.suggestions_provider = Some(provider);
        self
    }

    pub fn suggestions_provider(&self) -> Option<&dyn SuggestionProvider<S>> {
        self.suggestions_provider.as_ref()
    }

    pub fn get_type(&self) -> &dyn ArgumentType<T> {
        &self.type_
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    // final ArgumentCommandNode<S, T> result = new ArgumentCommandNode<>(getName(), getType(), getCommand(), getRequirement(), getRedirect(), getRedirectModifier(), isFork(), getSuggestionsProvider());

    // for (final CommandNode<S> argument : getArguments()) {
    // 	result.addChild(argument);
    // }

    // return result;
    pub fn build(self) -> ArgumentCommandNode<S, T> {
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
