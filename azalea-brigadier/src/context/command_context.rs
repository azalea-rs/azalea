use std::{
    any::Any,
    collections::HashMap,
    fmt::{self, Debug},
    rc::Rc,
    sync::Arc,
};

use parking_lot::RwLock;

use super::{ParsedArgument, parsed_command_node::ParsedCommandNode, string_range::StringRange};
use crate::{
    modifier::RedirectModifier,
    tree::{Command, CommandNode},
};

/// A built `CommandContextBuilder`.
pub struct CommandContext<S> {
    pub source: Arc<S>,
    pub(super) input: String,
    pub(super) arguments: HashMap<String, ParsedArgument>,
    pub(super) command: Command<S>,
    pub(super) root_node: Arc<RwLock<CommandNode<S>>>,
    pub(super) nodes: Vec<ParsedCommandNode<S>>,
    pub(super) range: StringRange,
    pub(super) child: Option<Rc<CommandContext<S>>>,
    pub(super) modifier: Option<Arc<RedirectModifier<S>>>,
    pub(super) forks: bool,
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
            range: self.range,
            child: self.child.clone(),
            modifier: self.modifier.clone(),
            forks: self.forks,
        }
    }
}

impl<S> Debug for CommandContext<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    pub fn copy_for(&self, source: Arc<S>) -> Self {
        if Arc::ptr_eq(&source, &self.source) {
            // fast path
            return self.clone();
        }

        CommandContext {
            source,
            input: self.input.clone(),
            arguments: self.arguments.clone(),
            command: self.command.clone(),
            root_node: self.root_node.clone(),
            nodes: self.nodes.clone(),
            range: self.range,
            child: self.child.clone(),
            modifier: self.modifier.clone(),
            forks: self.forks,
        }
    }

    pub fn child(&self) -> Option<&CommandContext<S>> {
        self.child.as_ref().map(|c| c.as_ref())
    }

    pub fn last_child(&self) -> &CommandContext<S> {
        let mut result = self;
        while let Some(child) = result.child() {
            result = child;
        }
        result
    }

    pub fn command(&self) -> &Command<S> {
        &self.command
    }

    pub fn argument(&self, name: &str) -> Option<&dyn Any> {
        let argument = self.arguments.get(name);
        argument.map(|a| a.result.as_ref())
    }

    pub fn redirect_modifier(&self) -> Option<&RedirectModifier<S>> {
        self.modifier.as_ref().map(|m| m.as_ref())
    }

    pub fn range(&self) -> &StringRange {
        &self.range
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn root_node(&self) -> &Arc<RwLock<CommandNode<S>>> {
        &self.root_node
    }

    pub fn nodes(&self) -> &[ParsedCommandNode<S>] {
        &self.nodes
    }

    pub fn has_nodes(&self) -> bool {
        !self.nodes.is_empty()
    }

    pub fn is_forked(&self) -> bool {
        self.forks
    }
}
