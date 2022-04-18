use std::{any::Any, cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

use crate::{
    dispatcher::CommandDispatcher,
    modifier::RedirectModifier,
    string_range::StringRange,
    tree::{CommandNode, ParsedCommandNode},
};

pub struct CommandContextBuilder<S> {
    pub arguments: HashMap<String, ParsedArgument>,
    pub root: Rc<RefCell<CommandNode<S>>>,
    pub nodes: Vec<ParsedCommandNode<S>>,
    pub dispatcher: Rc<CommandDispatcher<S>>,
    pub source: Rc<S>,
    pub command: Option<Rc<dyn Fn(&CommandContext<S>) -> i32>>,
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
            forks: self.forks.clone(),
        }
    }
}

impl<S> CommandContextBuilder<S> {
    // CommandDispatcher<S> dispatcher, final S source, final CommandNode<S> rootNode, final int start
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
            // rootNode,
            // start,
            child: None,
            modifier: None,
            forks: false,
        }
    }

    pub fn with_command(
        &mut self,
        command: &Option<Rc<dyn Fn(&CommandContext<S>) -> i32>>,
    ) -> &Self {
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

#[derive(Clone)]
pub struct ParsedArgument {
    pub range: StringRange,
    pub result: Rc<dyn Any>,
}

/// A built `CommandContextBuilder`.
pub struct CommandContext<S> {
    pub source: Rc<S>,
    pub input: String,
    pub arguments: HashMap<String, ParsedArgument>,
    pub command: Option<Rc<dyn Fn(&CommandContext<S>) -> i32>>,
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
            forks: self.forks.clone(),
        }
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
