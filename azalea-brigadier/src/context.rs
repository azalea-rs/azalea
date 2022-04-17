use std::{any::Any, cell::RefCell, collections::HashMap, fmt::Debug, ptr, rc::Rc};

use crate::{
    dispatcher::CommandDispatcher, modifier::RedirectModifier, string_range::StringRange,
    tree::CommandNode,
};

#[derive(Clone)]
pub struct CommandContextBuilder<S: Any + Clone> {
    pub arguments: HashMap<String, ParsedArgument>,
    pub root: Rc<RefCell<CommandNode<S>>>,
    pub nodes: Vec<Rc<CommandNode<S>>>,
    pub dispatcher: Rc<CommandDispatcher<S>>,
    pub source: Rc<S>,
    pub command: Option<Rc<dyn Fn(&CommandContext<S>) -> i32>>,
    pub child: Option<Rc<CommandContextBuilder<S>>>,
    pub range: StringRange,
    pub modifier: Option<Rc<dyn RedirectModifier<S>>>,
    pub forks: bool,
}

impl<S: Any + Clone> CommandContextBuilder<S> {
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
    pub fn with_node(&mut self, node: Rc<CommandNode<S>>, range: StringRange) -> &Self {
        self.nodes.push(node.clone());
        self.range = StringRange::encompassing(&self.range, &range);
        self.modifier = node.modifier.clone();
        self.forks = node.forks;
        self
    }

    pub fn build(&self, input: &str) -> CommandContext<S> {
        CommandContext {
            arguments: self.arguments.clone(),
            root_node: self.root.clone(),
            nodes: self.nodes.clone(),
            source: self.source.clone(),
            command: self.command.clone(),
            child: self.child.clone().map(|c| Rc::new(c.build(&input))),
            range: self.range.clone(),
            forks: self.forks,
            modifier: self.modifier.clone(),
            input: input.to_string(),
        }
    }
}

impl<S: Any + Clone> Debug for CommandContextBuilder<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandContextBuilder")
            // .field("arguments", &self.arguments)
            .field("root", &self.root)
            .field("nodes", &self.nodes)
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

#[derive(Clone)]
/// A built `CommandContextBuilder`.
pub struct CommandContext<S: Any + Clone> {
    pub source: Rc<S>,
    pub input: String,
    pub arguments: HashMap<String, ParsedArgument>,
    pub command: Option<Rc<dyn Fn(&CommandContext<S>) -> i32>>,
    pub root_node: Rc<RefCell<CommandNode<S>>>,
    pub nodes: Vec<Rc<CommandNode<S>>>,
    pub range: StringRange,
    pub child: Option<Rc<CommandContext<S>>>,
    pub modifier: Option<Rc<dyn RedirectModifier<S>>>,
    pub forks: bool,
}

impl<S: Any + Clone> CommandContext<S> {
    pub fn copy_for(&self, source: Rc<S>) -> Self {
        if Rc::ptr_eq(&source, &self.source) {
            return self.clone();
        }
        return CommandContext {
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
        };
    }

    pub fn has_nodes(&self) -> bool {
        return !self.nodes.is_empty();
    }

    pub fn argument(&self, name: &str) -> Option<Rc<dyn Any>> {
        let argument = self.arguments.get(name);
        argument.map(|a| a.result.clone())
    }
}
