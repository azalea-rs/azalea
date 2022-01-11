use std::collections::HashMap;

use crate::{
    arguments::argument_type::ArgumentType, command::Command,
    command_dispatcher::CommandDispatcher, redirect_modifier::RedirectModifier,
    tree::command_node::CommandNode,
};

use super::{
    command_context::CommandContext, parsed_argument::ParsedArgument,
    parsed_command_node::ParsedCommandNode, string_range::StringRange,
    suggestion_context::SuggestionContext,
};

// public class CommandContextBuilder<S> {
//     private final Map<String, ParsedArgument<S, ?>> arguments = new LinkedHashMap<>();
//     private final CommandNode<S> rootNode;
//     private final List<ParsedCommandNode<S>> nodes = new ArrayList<>();
//     private final CommandDispatcher<S> dispatcher;
//     private S source;
//     private Command<S> command;
//     private CommandContextBuilder<S> child;
//     private StringRange range;
//     private RedirectModifier<S> modifier = null;
//     private boolean forks;

#[derive(Clone)]
pub struct CommandContextBuilder<S> {
    arguments: HashMap<String, ParsedArgument<dyn ArgumentType<dyn ArgumentResult>>>,
    root_node: dyn CommandNode<S>,
    nodes: Vec<ParsedCommandNode<S>>,
    dispatcher: CommandDispatcher<S>,
    source: S,
    command: Box<dyn Command<S>>,
    child: Option<CommandContextBuilder<S>>,
    range: StringRange,
    modifier: Option<Box<dyn RedirectModifier<S>>>,
    forks: bool,
}

// public CommandContextBuilder(final CommandDispatcher<S> dispatcher, final S source, final CommandNode<S> rootNode, final int start) {
// 	this.rootNode = rootNode;
// 	this.dispatcher = dispatcher;
// 	this.source = source;
// 	this.range = StringRange.at(start);
// }

impl<S> CommandContextBuilder<S> {
    pub fn new(
        dispatcher: CommandDispatcher<S>,
        source: S,
        root_node: dyn CommandNode<S>,
        start: usize,
    ) -> Self {
        Self {
            root_node,
            dispatcher,
            source,
            range: StringRange::at(start),
            ..Default::default()
        }
    }

    pub fn with_source(mut self, source: S) -> Self {
        self.source = source;
        self
    }

    pub fn source(&self) -> &S {
        &self.source
    }

    pub fn root_node(&self) -> &dyn CommandNode<S> {
        &self.root_node
    }

    pub fn with_argument(
        mut self,
        name: String,
        argument: ParsedArgument<dyn ArgumentType<dyn ArgumentResult>>,
    ) -> Self {
        self.arguments.insert(name, argument);
        self
    }

    pub fn arguments(
        &self,
    ) -> &HashMap<String, ParsedArgument<dyn ArgumentType<dyn ArgumentResult>>> {
        &self.arguments
    }

    pub fn with_command(mut self, command: Box<dyn Command<S>>) -> Self {
        self.command = command;
        self
    }

    pub fn with_node(mut self, node: dyn CommandNode<S>, range: StringRange) -> Self {
        self.nodes.push(ParsedCommandNode::new(node, range));
        self.range = StringRange::encompassing(&self.range, &range);
        self.modifier = node.redirect_modifier();
        self.forks = node.is_fork();
        self
    }

    pub fn with_child(mut self, child: CommandContextBuilder<S>) -> Self {
        self.child = Some(child);
        self
    }

    pub fn child(&self) -> Option<&CommandContextBuilder<S>> {
        self.child.as_ref()
    }

    pub fn last_child(&self) -> Option<&CommandContextBuilder<S>> {
        let mut result = self;
        while let Some(child) = result.child() {
            result = child;
        }
        Some(result)
    }

    pub fn command(&self) -> &dyn Command<S> {
        &*self.command
    }

    pub fn nodes(&self) -> &Vec<ParsedCommandNode<S>> {
        &self.nodes
    }

    pub fn build(self, input: &str) -> CommandContext<S> {
        CommandContext {
            source: self.source,
            input,
            arguments: self.arguments,
            command: self.command,
            root_node: self.root_node,
            nodes: self.nodes,
            range: self.range,
            child: self.child.map(|child| child.build(input)),
            modifier: self.modifier,
            forks: self.forks,
        }
    }

    pub fn dispatcher(&self) -> &CommandDispatcher<S> {
        &self.dispatcher
    }

    pub fn range(&self) -> &StringRange {
        &self.range
    }

    pub fn find_suggestion_context(&self, cursor: i32) -> Result<SuggestionContext<S>, String> {
        if self.range.start() <= cursor {
            if self.range.end() < cursor {
                if let Some(child) = self.child() {
                    child.find_suggestion_context(cursor);
                } else if !self.nodes.is_empty() {
                    let last = self.nodes.last().unwrap();
                    let end = last.range().end() + 1;
                    return SuggestionContext::new(last.node(), end);
                } else {
                    return SuggestionContext::new(self.root_node, self.range.start());
                }
            } else {
                let prev = self.root_node;
                for node in &self.nodes {
                    let node_range = node.range();
                    if node_range.start() <= cursor && cursor <= node_range.end() {
                        return SuggestionContext::new(prev, node_range.start());
                    }
                    prev = node.node();
                }
                if prev.is_none() {
                    return Err(String::from("Can't find node before cursor"));
                }
                return SuggestionContext::new(prev.unwrap(), self.range.start());
            }
        }
        Err(String::from("Can't find node before cursor"))
    }
}
