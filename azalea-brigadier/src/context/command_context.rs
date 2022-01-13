use super::{
    parsed_argument::ParsedArgument, parsed_command_node::ParsedCommandNode,
    string_range::StringRange,
};
use crate::{
    arguments::argument_type::ArgumentType, command::Command, redirect_modifier::RedirectModifier,
    tree::command_node::CommandNode,
};
use std::{any::Any, collections::HashMap};

pub struct CommandContext<'a, S> {
    source: S,
    input: String,
    command: &'a dyn Command<S>,
    arguments: HashMap<String, ParsedArgument<Box<dyn Any>>>,
    root_node: &'a dyn CommandNode<S>,
    nodes: Vec<ParsedCommandNode<S>>,
    range: StringRange,
    child: Option<&'a CommandContext<'a, S>>,
    modifier: Option<&'a dyn RedirectModifier<S>>,
    forks: bool,
}

impl<S> CommandContext<'_, S>
where
    S: PartialEq,
{
    pub fn clone_for(&self, source: S) -> Self {
        if self.source == source {
            return *self;
        }
        Self {
            source,
            input: self.input.clone(),
            command: self.command.clone(),
            arguments: self.arguments.clone(),
            root_node: self.root_node.clone(),
            nodes: self.nodes.clone(),
            range: self.range.clone(),
            child: self.child.clone(),
            modifier: self.modifier.clone(),
            forks: self.forks,
        }
    }

    fn child(&self) -> &Option<CommandContext<S>> {
        &self.child
    }

    fn last_child(&self) -> &CommandContext<S> {
        let mut result = self;
        while result.child.is_some() {
            result = result.child.as_ref().unwrap();
        }
        result
    }

    fn command(&self) -> &dyn Command<S> {
        &self.command
    }

    fn source(&self) -> &S {
        &self.source
    }

    // public <V> V getArgument(final String name, final Class<V> clazz) {
    //     final ParsedArgument<S, ?> argument = arguments.get(name);

    //     if (argument == null) {
    //         throw new IllegalArgumentException("No such argument '" + name + "' exists on this command");
    //     }

    //     final Object result = argument.getResult();
    //     if (PRIMITIVE_TO_WRAPPER.getOrDefault(clazz, clazz).isAssignableFrom(result.getClass())) {
    //         return (V) result;
    //     } else {
    //         throw new IllegalArgumentException("Argument '" + name + "' is defined as " + result.getClass().getSimpleName() + ", not " + clazz);
    //     }
    // }
    fn get_argument<V>(&self, name: &str) -> Result<V, String> {
        let argument = self.arguments.get(name);

        if argument.is_none() {
            return Err(format!(
                "No such argument '{}' exists on this command",
                name
            ));
        }

        let result = argument.unwrap().result();
        Ok(result)
    }
}
