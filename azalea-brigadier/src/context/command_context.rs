use super::{parsed_command_node::ParsedCommandNode, string_range::StringRange, ParsedArgument};
use crate::{
    modifier::RedirectModifier,
    tree::{Command, CommandNode},
};
use std::{any::Any, cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

/// A built `CommandContextBuilder`.
pub struct CommandContext<S> {
    pub source: Rc<S>,
    pub input: String,
    pub arguments: HashMap<String, ParsedArgument>,
    pub command: Command<S>,
    pub root_node: Rc<RefCell<CommandNode<S>>>,
    pub nodes: Vec<ParsedCommandNode<S>>,
    pub range: StringRange,
    pub child: Option<Rc<CommandContext<S>>>,
    pub modifier: Option<Rc<RedirectModifier<S>>>,
    pub forks: bool,
}

impl<S> Clone for CommandContext<S> {
    fn clone(&self) -> Self {
        Self {
            source: self.source.clone(),
            input: self.input.clone(),
            arguments: self.arguments.clone(),
            command: self.command.clone(),
            root_node: self.root_node.clone(),
            nodes: self.nodes.clone(),
            range: self.range.clone(),
            child: self.child.clone(),
            modifier: self.modifier.clone(),
            forks: self.forks,
        }
    }
}

impl<S> Debug for CommandContext<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandContext")
            // .field("source", &self.source)
            .field("input", &self.input)
            // .field("arguments", &self.arguments)
            // .field("command", &self.command)
            // .field("root_node", &self.root_node)
            // .field("nodes", &self.nodes)
            .field("range", &self.range)
            .field("child", &self.child)
            // .field("modifier", &self.modifier)
            .field("forks", &self.forks)
            .finish()
    }
}

impl<S> CommandContext<S> {
    pub fn copy_for(&self, source: Rc<S>) -> Self {
        if Rc::ptr_eq(&source, &self.source) {
            return self.clone();
        }
        CommandContext {
            source,
            input: self.input.clone(),
            arguments: self.arguments.clone(),
            command: self.command.clone(),
            root_node: self.root_node.clone(),
            nodes: self.nodes.clone(),
            range: self.range.clone(),
            child: self.child.clone(),
            modifier: self.modifier.clone(),
            forks: self.forks,
        }
    }

    pub fn has_nodes(&self) -> bool {
        !self.nodes.is_empty()
    }

    pub fn argument(&self, name: &str) -> Option<Rc<dyn Any>> {
        let argument = self.arguments.get(name);
        argument.map(|a| a.result.clone())
    }
}
