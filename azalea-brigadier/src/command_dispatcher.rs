use crate::{
    builder::argument_builder::ArgumentBuilder,
    context::{CommandContext, CommandContextBuilder},
    exceptions::{BuiltInExceptions, CommandSyntaxException},
    parse_results::ParseResults,
    string_reader::StringReader,
    tree::CommandNode,
};
use std::{cell::RefCell, cmp::Ordering, collections::HashMap, marker::PhantomData, mem, rc::Rc};

#[derive(Default)]
pub struct CommandDispatcher<S> {
    pub root: Rc<RefCell<CommandNode<S>>>,
    _marker: PhantomData<S>,
}

impl<S> CommandDispatcher<S> {
    pub fn new() -> Self {
        Self {
            root: Rc::new(RefCell::new(CommandNode::default())),
            _marker: PhantomData,
        }
    }

    pub fn register(&mut self, node: ArgumentBuilder<S>) -> Rc<RefCell<CommandNode<S>>> {
        let build = Rc::new(RefCell::new(node.build()));
        self.root.borrow_mut().add_child(&build);
        build
    }

    pub fn parse(&self, command: StringReader, source: Rc<S>) -> ParseResults<S> {
        let context = CommandContextBuilder::new(
            Rc::new(self.clone()),
            source,
            self.root.clone(),
            command.cursor(),
        );
        self.parse_nodes(&self.root, &command, context).unwrap()
    }

    fn parse_nodes(
        &self,
        node: &Rc<RefCell<CommandNode<S>>>,
        original_reader: &StringReader,
        context_so_far: CommandContextBuilder<S>,
    ) -> Result<ParseResults<S>, CommandSyntaxException> {
        let source = context_so_far.source.clone();
        let mut errors = HashMap::<Rc<CommandNode<S>>, CommandSyntaxException>::new();
        let mut potentials: Vec<ParseResults<S>> = vec![];
        let cursor = original_reader.cursor();

        for child in node
            .borrow()
            .get_relevant_nodes(&mut original_reader.clone())
        {
            if !child.borrow().can_use(source.clone()) {
                continue;
            }
            let mut context = context_so_far.clone();
            let mut reader = original_reader.clone();

            let parse_with_context_result =
                child.borrow().parse_with_context(&mut reader, &mut context);
            if let Err(ex) = parse_with_context_result {
                errors.insert(
                    Rc::new((*child.borrow()).clone()),
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
                    Rc::new((*child.borrow()).clone()),
                    BuiltInExceptions::DispatcherExpectedArgumentSeparator
                        .create_with_context(&reader),
                );
                reader.cursor = cursor;
                continue;
            }

            context.with_command(&child.borrow().command);
            if reader.can_read_length(if child.borrow().redirect.is_none() {
                2
            } else {
                1
            }) {
                reader.skip();
                if let Some(redirect) = &child.borrow().redirect {
                    let child_context = CommandContextBuilder::new(
                        Rc::new(self.clone()),
                        source,
                        redirect.clone(),
                        reader.cursor,
                    );
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
                })
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

    pub fn execute(
        &self,
        input: StringReader,
        source: Rc<S>,
    ) -> Result<i32, CommandSyntaxException> {
        let parse = self.parse(input, source);
        Self::execute_parsed(parse)
    }

    pub fn add_paths(
        node: Rc<RefCell<CommandNode<S>>>,
        result: &mut Vec<Vec<Rc<RefCell<CommandNode<S>>>>>,
        parents: Vec<Rc<RefCell<CommandNode<S>>>>,
    ) {
        let mut current = parents;
        current.push(node.clone());
        result.push(current.clone());

        for child in node.borrow().children.values() {
            Self::add_paths(child.clone(), result, current.clone());
        }
    }

    pub fn get_path(&self, target: CommandNode<S>) -> Vec<String> {
        let rc_target = Rc::new(RefCell::new(target));
        let mut nodes: Vec<Vec<Rc<RefCell<CommandNode<S>>>>> = Vec::new();
        Self::add_paths(self.root.clone(), &mut nodes, vec![]);

        for list in nodes {
            if *list.last().expect("Nothing in list").borrow() == *rc_target.borrow() {
                let mut result: Vec<String> = Vec::with_capacity(list.len());
                for node in list {
                    if node != self.root {
                        result.push(node.borrow().name().to_string());
                    }
                }
                return result;
            }
        }
        vec![]
    }

    pub fn find_node(&self, path: &[&str]) -> Option<Rc<RefCell<CommandNode<S>>>> {
        let mut node = self.root.clone();
        for name in path {
            if let Some(child) = node.clone().borrow().child(name) {
                node = child
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
            for context in contexts.iter() {
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

                    // TODO: allow context_command to error and handle those errors
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
}

impl<S> Clone for CommandDispatcher<S> {
    fn clone(&self) -> Self {
        Self {
            root: self.root.clone(),
            _marker: PhantomData,
        }
    }
}
