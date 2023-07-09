use parking_lot::RwLock;

use super::{
    command_context::CommandContext, parsed_command_node::ParsedCommandNode,
    string_range::StringRange, ParsedArgument,
};
use crate::{
    command_dispatcher::CommandDispatcher,
    modifier::RedirectModifier,
    tree::{Command, CommandNode},
};
use std::{collections::HashMap, fmt::Debug, rc::Rc, sync::Arc};

pub struct CommandContextBuilder<'a, S> {
    pub arguments: HashMap<String, ParsedArgument>,
    pub root: Arc<RwLock<CommandNode<S>>>,
    pub nodes: Vec<ParsedCommandNode<S>>,
    pub dispatcher: &'a CommandDispatcher<S>,
    pub source: Arc<S>,
    pub command: Command<S>,
    pub child: Option<Rc<CommandContextBuilder<'a, S>>>,
    pub range: StringRange,
    pub modifier: Option<Arc<RedirectModifier<S>>>,
    pub forks: bool,
}

impl<S> Clone for CommandContextBuilder<'_, S> {
    fn clone(&self) -> Self {
        Self {
            arguments: self.arguments.clone(),
            root: self.root.clone(),
            nodes: self.nodes.clone(),
            dispatcher: self.dispatcher,
            source: self.source.clone(),
            command: self.command.clone(),
            child: self.child.clone(),
            range: self.range.clone(),
            modifier: self.modifier.clone(),
            forks: self.forks,
        }
    }
}

impl<'a, S> CommandContextBuilder<'a, S> {
    pub fn new(
        dispatcher: &'a CommandDispatcher<S>,
        source: Arc<S>,
        root_node: Arc<RwLock<CommandNode<S>>>,
        start: usize,
    ) -> Self {
        Self {
            arguments: HashMap::new(),
            root: root_node,
            source,
            range: StringRange::at(start),
            command: None,
            dispatcher,
            nodes: vec![],
            child: None,
            modifier: None,
            forks: false,
        }
    }

    pub fn with_command(&mut self, command: &Command<S>) -> &Self {
        self.command = command.clone();
        self
    }
    pub fn with_child(&mut self, child: Rc<CommandContextBuilder<'a, S>>) -> &Self {
        self.child = Some(child);
        self
    }
    pub fn with_argument(&mut self, name: &str, argument: ParsedArgument) -> &Self {
        self.arguments.insert(name.to_string(), argument);
        self
    }
    pub fn with_node(&mut self, node: Arc<RwLock<CommandNode<S>>>, range: StringRange) -> &Self {
        self.nodes.push(ParsedCommandNode {
            node: node.clone(),
            range: range.clone(),
        });
        self.range = StringRange::encompassing(&self.range, &range);
        self.modifier = node.read().modifier.clone();
        self.forks = node.read().forks;
        self
    }

    pub fn build(&self, input: &str) -> CommandContext<S> {
        CommandContext {
            arguments: self.arguments.clone(),
            root_node: self.root.clone(),
            nodes: self.nodes.clone(),
            source: self.source.clone(),
            command: self.command.clone(),
            child: self.child.clone().map(|c| Rc::new(c.build(input))),
            range: self.range.clone(),
            forks: self.forks,
            modifier: self.modifier.clone(),
            input: input.to_string(),
        }
    }
}

impl<S> Debug for CommandContextBuilder<'_, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandContextBuilder")
            // .field("arguments", &self.arguments)
            .field("root", &self.root)
            // .field("nodes", &self.nodes)
            // .field("dispatcher", &self.dispatcher)
            // .field("source", &self.source)
            // .field("command", &self.command)
            .field("child", &self.child)
            .field("range", &self.range)
            // .field("modifier", &self.modifier)
            .field("forks", &self.forks)
            .finish()
    }
}
