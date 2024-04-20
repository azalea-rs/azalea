use parking_lot::RwLock;

use super::{
    command_context::CommandContext, parsed_command_node::ParsedCommandNode,
    string_range::StringRange, suggestion_context::SuggestionContext, ParsedArgument,
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
            range: self.range,
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
        self.command.clone_from(command);
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
            range,
        });
        self.range = StringRange::encompassing(&self.range, &range);
        self.modifier.clone_from(&node.read().modifier);
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
            range: self.range,
            forks: self.forks,
            modifier: self.modifier.clone(),
            input: input.to_string(),
        }
    }

    pub fn find_suggestion_context(&self, cursor: usize) -> SuggestionContext<S> {
        if self.range.start() > cursor {
            panic!("Can't find node before cursor");
        }

        if self.range.end() < cursor {
            if let Some(child) = &self.child {
                child.find_suggestion_context(cursor)
            } else if let Some(last) = self.nodes.last() {
                SuggestionContext {
                    parent: Arc::clone(&last.node),
                    start_pos: last.range.end() + 1,
                }
            } else {
                SuggestionContext {
                    parent: Arc::clone(&self.root),
                    start_pos: self.range.start(),
                }
            }
        } else {
            let mut prev = &self.root;
            for node in &self.nodes {
                if node.range.start() <= cursor && cursor <= node.range.end() {
                    return SuggestionContext {
                        parent: Arc::clone(prev),
                        start_pos: node.range.start(),
                    };
                }
                prev = &node.node;
            }
            SuggestionContext {
                parent: Arc::clone(prev),
                start_pos: self.range.start(),
            }
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
