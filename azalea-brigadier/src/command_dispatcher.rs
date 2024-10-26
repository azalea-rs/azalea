use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    mem,
    rc::Rc,
    sync::Arc,
};

use parking_lot::RwLock;

use crate::{
    builder::argument_builder::ArgumentBuilder,
    context::{CommandContext, CommandContextBuilder},
    exceptions::{BuiltInExceptions, CommandSyntaxException},
    parse_results::ParseResults,
    string_reader::StringReader,
    suggestion::{Suggestions, SuggestionsBuilder},
    tree::CommandNode,
};

/// The root of the command tree. You need to make this to register commands.
///
/// ```
/// # use azalea_brigadier::prelude::*;
/// # struct CommandSource;
/// let mut subject = CommandDispatcher::<CommandSource>::new();
/// ```
pub struct CommandDispatcher<S>
where
    Self: Sync + Send,
{
    pub root: Arc<RwLock<CommandNode<S>>>,
}

impl<S> CommandDispatcher<S> {
    pub fn new() -> Self {
        Self {
            root: Arc::new(RwLock::new(CommandNode::default())),
        }
    }

    /// Add a new node to the root.
    ///
    /// ```
    /// # use azalea_brigadier::prelude::*;
    /// # let mut subject = CommandDispatcher::<()>::new();
    /// subject.register(literal("foo").executes(|_| 42));
    /// ```
    pub fn register(&mut self, node: ArgumentBuilder<S>) -> Arc<RwLock<CommandNode<S>>> {
        let build = Arc::new(RwLock::new(node.build()));
        self.root.write().add_child(&build);
        build
    }

    pub fn parse(&self, command: StringReader, source: S) -> ParseResults<S> {
        let source = Arc::new(source);

        let context = CommandContextBuilder::new(self, source, self.root.clone(), command.cursor());
        self.parse_nodes(&self.root, &command, context).unwrap()
    }

    fn parse_nodes<'a>(
        &'a self,
        node: &Arc<RwLock<CommandNode<S>>>,
        original_reader: &StringReader,
        context_so_far: CommandContextBuilder<'a, S>,
    ) -> Result<ParseResults<'a, S>, CommandSyntaxException> {
        let source = context_so_far.source.clone();
        #[allow(clippy::mutable_key_type)] // this is fine because we don't mutate the key
        let mut errors = HashMap::<Rc<CommandNode<S>>, CommandSyntaxException>::new();
        let mut potentials: Vec<ParseResults<S>> = vec![];
        let cursor = original_reader.cursor();

        for child in node.read().get_relevant_nodes(&mut original_reader.clone()) {
            if !child.read().can_use(&source) {
                continue;
            }
            let mut context = context_so_far.clone();
            let mut reader = original_reader.clone();

            let parse_with_context_result =
                child.read().parse_with_context(&mut reader, &mut context);
            if let Err(ex) = parse_with_context_result {
                errors.insert(
                    Rc::new((*child.read()).clone()),
                    BuiltInExceptions::DispatcherParseException {
                        message: ex.message(),
                    }
                    .create_with_context(&reader),
                );
                reader.cursor = cursor;
                continue;
            }
            if reader.can_read() && reader.peek() != ' ' {
                errors.insert(
                    Rc::new((*child.read()).clone()),
                    BuiltInExceptions::DispatcherExpectedArgumentSeparator
                        .create_with_context(&reader),
                );
                reader.cursor = cursor;
                continue;
            }

            context.with_command(&child.read().command);
            if reader.can_read_length(if child.read().redirect.is_none() {
                2
            } else {
                1
            }) {
                reader.skip();
                if let Some(redirect) = &child.read().redirect {
                    let child_context =
                        CommandContextBuilder::new(self, source, redirect.clone(), reader.cursor);
                    let parse = self
                        .parse_nodes(redirect, &reader, child_context)
                        .expect("Parsing nodes failed");
                    context.with_child(Rc::new(parse.context));
                    return Ok(ParseResults {
                        context,
                        reader: parse.reader,
                        exceptions: parse.exceptions,
                    });
                } else {
                    let parse = self
                        .parse_nodes(&child, &reader, context)
                        .expect("Parsing nodes failed");
                    potentials.push(parse);
                }
            } else {
                potentials.push(ParseResults {
                    context,
                    reader,
                    exceptions: HashMap::new(),
                });
            }
        }

        if !potentials.is_empty() {
            if potentials.len() > 1 {
                potentials.sort_by(|a, b| {
                    if !a.reader.can_read() && b.reader.can_read() {
                        return Ordering::Less;
                    };
                    if a.reader.can_read() && !b.reader.can_read() {
                        return Ordering::Greater;
                    };
                    if a.exceptions.is_empty() && !b.exceptions.is_empty() {
                        return Ordering::Less;
                    };
                    if !a.exceptions.is_empty() && b.exceptions.is_empty() {
                        return Ordering::Greater;
                    };
                    Ordering::Equal
                });
            }
            let best_potential = potentials.into_iter().next().unwrap();
            return Ok(best_potential);
        }

        Ok(ParseResults {
            context: context_so_far,
            reader: original_reader.clone(),
            exceptions: errors,
        })
    }

    /// Parse and execute the command using the given input and context. The
    /// number returned depends on the command, and may not be of significance.
    ///
    /// This is a shortcut for `Self::parse` and `Self::execute_parsed`.
    pub fn execute(
        &self,
        input: impl Into<StringReader>,
        source: S,
    ) -> Result<i32, CommandSyntaxException> {
        let input = input.into();

        let parse = self.parse(input, source);
        Self::execute_parsed(parse)
    }

    pub fn add_paths(
        node: Arc<RwLock<CommandNode<S>>>,
        result: &mut Vec<Vec<Arc<RwLock<CommandNode<S>>>>>,
        parents: Vec<Arc<RwLock<CommandNode<S>>>>,
    ) {
        let mut current = parents;
        current.push(node.clone());
        result.push(current.clone());

        for child in node.read().children.values() {
            Self::add_paths(child.clone(), result, current.clone());
        }
    }

    pub fn get_path(&self, target: CommandNode<S>) -> Vec<String> {
        let rc_target = Arc::new(RwLock::new(target));
        let mut nodes: Vec<Vec<Arc<RwLock<CommandNode<S>>>>> = Vec::new();
        Self::add_paths(self.root.clone(), &mut nodes, vec![]);

        for list in nodes {
            if *list.last().expect("Nothing in list").read() == *rc_target.read() {
                let mut result: Vec<String> = Vec::with_capacity(list.len());
                for node in list {
                    if !Arc::ptr_eq(&node, &self.root) {
                        result.push(node.read().name().to_string());
                    }
                }
                return result;
            }
        }
        vec![]
    }

    pub fn find_node(&self, path: &[&str]) -> Option<Arc<RwLock<CommandNode<S>>>> {
        let mut node = self.root.clone();
        for name in path {
            if let Some(child) = node.clone().read().child(name) {
                node = child;
            } else {
                return None;
            }
        }
        Some(node)
    }

    /// Executes a given pre-parsed command.
    pub fn execute_parsed(parse: ParseResults<S>) -> Result<i32, CommandSyntaxException> {
        if parse.reader.can_read() {
            if parse.exceptions.len() == 1 {
                return Err(parse.exceptions.values().next().unwrap().clone());
            }
            if parse.context.range.is_empty() {
                return Err(
                    BuiltInExceptions::DispatcherUnknownCommand.create_with_context(&parse.reader)
                );
            }
            return Err(
                BuiltInExceptions::DispatcherUnknownArgument.create_with_context(&parse.reader)
            );
        }
        let mut result = 0i32;
        let mut successful_forks = 0;
        let mut forked = false;
        let mut found_command = false;
        let command = parse.reader.string();
        let original = parse.context.build(command);
        let mut contexts = vec![original];
        let mut next: Vec<CommandContext<S>> = vec![];

        while !contexts.is_empty() {
            for context in &contexts {
                let child = &context.child;
                if let Some(child) = child {
                    forked |= child.forks;
                    if child.has_nodes() {
                        found_command = true;
                        let modifier = &context.modifier;
                        if let Some(modifier) = modifier {
                            let results = modifier(context);
                            if let Ok(results) = results {
                                if !results.is_empty() {
                                    next.extend(results.iter().map(|s| child.copy_for(s.clone())));
                                }
                            } else {
                                // TODO
                                // self.consumer.on_command_complete(context, false, 0);
                                if !forked {
                                    return Err(results.err().unwrap());
                                }
                            }
                        } else {
                            next.push(child.copy_for(context.source.clone()));
                        }
                    }
                } else if let Some(context_command) = &context.command {
                    found_command = true;

                    let value = context_command(context);
                    result += value;
                    // consumer.on_command_complete(context, true, value);
                    successful_forks += 1;

                    // TODO: allow context_command to error and handle those
                    // errors
                }
            }

            // move next into contexts and clear next
            mem::swap(&mut contexts, &mut next);
            next.clear();
        }

        if !found_command {
            // consumer.on_command_complete(original, false, 0);
            return Err(
                BuiltInExceptions::DispatcherUnknownCommand.create_with_context(&parse.reader)
            );
        }

        // TODO: this is not how vanilla does it but it works
        Ok(if successful_forks >= 2 {
            successful_forks
        } else {
            result
        })
        // Ok(if forked { successful_forks } else { result })
    }

    pub fn get_all_usage(
        &self,
        node: &CommandNode<S>,
        source: &S,
        restricted: bool,
    ) -> Vec<String> {
        let mut result = vec![];
        self.get_all_usage_recursive(node, source, &mut result, "", restricted);
        result
    }

    fn get_all_usage_recursive(
        &self,
        node: &CommandNode<S>,
        source: &S,
        result: &mut Vec<String>,
        prefix: &str,
        restricted: bool,
    ) {
        if restricted && !node.can_use(source) {
            return;
        }
        if node.command.is_some() {
            result.push(prefix.to_owned());
        }
        if let Some(redirect) = &node.redirect {
            let redirect = if redirect.data_ptr() == self.root.data_ptr() {
                "...".to_string()
            } else {
                format!("-> {}", redirect.read().usage_text())
            };
            if prefix.is_empty() {
                result.push(format!("{} {redirect}", node.usage_text()));
            } else {
                result.push(format!("{prefix} {redirect}"));
            }
        } else {
            for child in node.children.values() {
                let child = child.read();
                self.get_all_usage_recursive(
                    &child,
                    source,
                    result,
                    if prefix.is_empty() {
                        child.usage_text()
                    } else {
                        format!("{prefix} {}", child.usage_text())
                    }
                    .as_str(),
                    restricted,
                );
            }
        }
    }

    /// Gets the possible executable commands from a specified node.
    ///
    /// You may use [`Self::root`] as a target to get usage data for the entire
    /// command tree.
    pub fn get_smart_usage(
        &self,
        node: &CommandNode<S>,
        source: &S,
    ) -> Vec<(Arc<RwLock<CommandNode<S>>>, String)> {
        let mut result = Vec::new();

        let optional = node.command.is_some();
        for child in node.children.values() {
            let usage = self.get_smart_usage_recursive(&child.read(), source, optional, false);
            if let Some(usage) = usage {
                result.push((child.clone(), usage));
            }
        }

        result
    }

    fn get_smart_usage_recursive(
        &self,
        node: &CommandNode<S>,
        source: &S,
        optional: bool,
        deep: bool,
    ) -> Option<String> {
        if !node.can_use(source) {
            return None;
        }

        let this = if optional {
            format!("[{}]", node.usage_text())
        } else {
            node.usage_text()
        };
        let child_optional = node.command.is_some();
        let open = if child_optional { "[" } else { "(" };
        let close = if child_optional { "]" } else { ")" };

        if deep {
            return Some(this);
        }

        if let Some(redirect) = &node.redirect {
            let redirect = if redirect.data_ptr() == self.root.data_ptr() {
                "...".to_string()
            } else {
                format!("-> {}", redirect.read().usage_text())
            };
            return Some(format!("{this} {redirect}"));
        }

        let children = node
            .children
            .values()
            .filter(|child| child.read().can_use(source))
            .collect::<Vec<_>>();
        match children.len().cmp(&1) {
            Ordering::Less => {}
            Ordering::Equal => {
                let usage = self.get_smart_usage_recursive(
                    &children[0].read(),
                    source,
                    child_optional,
                    child_optional,
                );
                if let Some(usage) = usage {
                    return Some(format!("{this} {usage}"));
                }
            }
            Ordering::Greater => {
                let mut child_usage = HashSet::new();
                for child in &children {
                    let usage =
                        self.get_smart_usage_recursive(&child.read(), source, child_optional, true);
                    if let Some(usage) = usage {
                        child_usage.insert(usage);
                    }
                }
                match child_usage.len().cmp(&1) {
                    Ordering::Less => {}
                    Ordering::Equal => {
                        let usage = child_usage.into_iter().next().unwrap();
                        let usage = if child_optional {
                            format!("[{}]", usage)
                        } else {
                            usage
                        };
                        return Some(format!("{this} {usage}"));
                    }
                    Ordering::Greater => {
                        let mut builder = String::new();
                        builder.push_str(open);
                        let mut count = 0;
                        for child in children {
                            if count > 0 {
                                builder.push('|');
                            }
                            builder.push_str(&child.read().usage_text());
                            count += 1;
                        }
                        if count > 0 {
                            builder.push_str(close);
                            return Some(format!("{this} {builder}"));
                        }
                    }
                }
            }
        }

        Some(this)
    }

    pub fn get_completion_suggestions(parse: ParseResults<S>) -> Suggestions {
        let cursor = parse.reader.total_length();
        Self::get_completion_suggestions_with_cursor(parse, cursor)
    }

    pub fn get_completion_suggestions_with_cursor(
        parse: ParseResults<S>,
        cursor: usize,
    ) -> Suggestions {
        let context = parse.context;

        let node_before_cursor = context.find_suggestion_context(cursor);
        let parent = node_before_cursor.parent;
        let start = usize::min(node_before_cursor.start_pos, cursor);

        let full_input = parse.reader.string();
        let truncated_input = full_input[..cursor].to_string();
        let truncated_input_lowercase = truncated_input.to_lowercase();

        let mut all_suggestions = Vec::new();
        for node in parent.read().children.values() {
            let suggestions = node.read().list_suggestions(
                context.build(&truncated_input),
                SuggestionsBuilder::new_with_lowercase(
                    &truncated_input,
                    &truncated_input_lowercase,
                    start,
                ),
            );
            all_suggestions.push(suggestions);
        }

        Suggestions::merge(full_input, &all_suggestions)
    }
}

impl<S> Default for CommandDispatcher<S> {
    fn default() -> Self {
        Self::new()
    }
}
