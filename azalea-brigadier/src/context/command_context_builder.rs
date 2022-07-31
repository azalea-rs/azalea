use super::{
    command_context::CommandContext, parsed_command_node::ParsedCommandNode,
    string_range::StringRange, ParsedArgument,
};
use crate::{
    command_dispatcher::CommandDispatcher,
    modifier::RedirectModifier,
    tree::{Command, CommandNode},
};
use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

pub struct CommandContextBuilder<S> {
    pub arguments: HashMap<String, ParsedArgument>,
    pub root: Rc<RefCell<CommandNode<S>>>,
    pub nodes: Vec<ParsedCommandNode<S>>,
    pub dispatcher: Rc<CommandDispatcher<S>>,
    pub source: Rc<S>,
    pub command: Command<S>,
    pub child: Option<Rc<CommandContextBuilder<S>>>,
    pub range: StringRange,
    pub modifier: Option<Rc<RedirectModifier<S>>>,
    pub forks: bool,
}

impl<S> Clone for CommandContextBuilder<S> {
    fn clone(&self) -> Self {
        Self {
            arguments: self.arguments.clone(),
            root: self.root.clone(),
            nodes: self.nodes.clone(),
            dispatcher: self.dispatcher.clone(),
            source: self.source.clone(),
            command: self.command.clone(),
            child: self.child.clone(),
            range: self.range.clone(),
            modifier: self.modifier.clone(),
            forks: self.forks,
        }
    }
}

impl<S> CommandContextBuilder<S> {
    pub fn new(
        dispatcher: Rc<CommandDispatcher<S>>,
        source: Rc<S>,
        root_node: Rc<RefCell<CommandNode<S>>>,
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
    pub fn with_child(&mut self, child: Rc<CommandContextBuilder<S>>) -> &Self {
        self.child = Some(child);
        self
    }
    pub fn with_argument(&mut self, name: &str, argument: ParsedArgument) -> &Self {
        self.arguments.insert(name.to_string(), argument);
        self
    }
    pub fn with_node(&mut self, node: Rc<RefCell<CommandNode<S>>>, range: StringRange) -> &Self {
        self.nodes.push(ParsedCommandNode {
            node: node.clone(),
            range: range.clone(),
        });
        self.range = StringRange::encompassing(&self.range, &range);
        self.modifier = node.borrow().modifier.clone();
        self.forks = node.borrow().forks;
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

impl<S> Debug for CommandContextBuilder<S> {
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
