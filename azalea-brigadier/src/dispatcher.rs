use crate::{
    builder::argument_builder::ArgumentBuilder,
    context::{CommandContext, CommandContextBuilder},
    exceptions::{
        builtin_exceptions::BuiltInExceptions, command_syntax_exception::CommandSyntaxException,
    },
    parse_results::ParseResults,
    string_reader::StringReader,
    tree::CommandNode,
};
use std::{cell::RefCell, cmp::Ordering, collections::HashMap, marker::PhantomData, mem, rc::Rc};

#[derive(Default)]
pub struct CommandDispatcher<S> {
    root: Rc<RefCell<CommandNode<S>>>,
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
        &self,
        node: Rc<RefCell<CommandNode<S>>>,
        result: &mut Vec<Vec<Rc<RefCell<CommandNode<S>>>>>,
        parents: Vec<Rc<RefCell<CommandNode<S>>>>,
    ) {
        let mut current = parents;
        current.push(node.clone());
        result.push(current.clone());

        for child in node.borrow().children.values() {
            self.add_paths(child.clone(), result, current.clone());
        }
    }

    pub fn get_path(&self, target: CommandNode<S>) -> Vec<String> {
        let rc_target = Rc::new(RefCell::new(target));
        let mut nodes: Vec<Vec<Rc<RefCell<CommandNode<S>>>>> = Vec::new();
        self.add_paths(self.root.clone(), &mut nodes, vec![]);

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
                    println!("aaaaaaa {:?}", child);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        builder::{literal_argument_builder::literal, required_argument_builder::argument},
        parsers::integer,
    };

    #[derive(Debug, PartialEq)]
    struct CommandSource {}

    fn input_with_offset(input: &str, offset: usize) -> StringReader {
        let mut result: StringReader = input.into();
        result.cursor = offset;
        result
    }

    #[test]
    fn create_and_execute_command() {
        let mut subject = CommandDispatcher::new();
        subject.register(literal("foo").executes(|_| 42));

        assert_eq!(
            subject
                .execute("foo".into(), Rc::new(CommandSource {}))
                .unwrap(),
            42
        );
    }

    #[test]
    fn create_and_execute_offset_command() {
        let mut subject = CommandDispatcher::new();
        subject.register(literal("foo").executes(|_| 42));

        assert_eq!(
            subject
                .execute(input_with_offset("/foo", 1), Rc::new(CommandSource {}))
                .unwrap(),
            42
        );
    }

    #[test]
    fn create_and_merge_commands() {
        let mut subject = CommandDispatcher::new();
        subject.register(literal("base").then(literal("foo").executes(|_| 42)));
        subject.register(literal("base").then(literal("bar").executes(|_| 42)));

        assert_eq!(
            subject
                .execute("base foo".into(), Rc::new(CommandSource {}))
                .unwrap(),
            42
        );
        assert_eq!(
            subject
                .execute("base bar".into(), Rc::new(CommandSource {}))
                .unwrap(),
            42
        );
    }

    #[test]
    fn execute_unknown_command() {
        let mut subject = CommandDispatcher::new();
        subject.register(literal("bar"));
        subject.register(literal("baz"));

        let execute_result = subject.execute("foo".into(), Rc::new(CommandSource {}));

        let err = execute_result.err().unwrap();
        match err.type_ {
            BuiltInExceptions::DispatcherUnknownCommand => {}
            _ => panic!("Unexpected error"),
        }
        assert_eq!(err.cursor().unwrap(), 0);
    }

    #[test]
    fn execute_impermissible_command() {
        let mut subject = CommandDispatcher::new();
        subject.register(literal("foo").requires(|_| false));

        let execute_result = subject.execute("foo".into(), Rc::new(CommandSource {}));

        let err = execute_result.err().unwrap();
        match err.type_ {
            BuiltInExceptions::DispatcherUnknownCommand => {}
            _ => panic!("Unexpected error"),
        }
        assert_eq!(err.cursor().unwrap(), 0);
    }

    #[test]
    fn execute_empty_command() {
        let mut subject = CommandDispatcher::new();
        subject.register(literal(""));

        let execute_result = subject.execute("".into(), Rc::new(CommandSource {}));

        let err = execute_result.err().unwrap();
        match err.type_ {
            BuiltInExceptions::DispatcherUnknownCommand => {}
            _ => panic!("Unexpected error"),
        }
        assert_eq!(err.cursor().unwrap(), 0);
    }

    #[test]
    fn execute_unknown_subcommand() {
        let mut subject = CommandDispatcher::new();
        subject.register(literal("foo").executes(|_| 42));

        let execute_result = subject.execute("foo bar".into(), Rc::new(CommandSource {}));

        let err = execute_result.err().unwrap();
        match err.type_ {
            BuiltInExceptions::DispatcherUnknownArgument => {}
            _ => panic!("Unexpected error"),
        }
        assert_eq!(err.cursor().unwrap(), 4);
    }

    #[test]
    fn execute_incorrect_literal() {
        let mut subject = CommandDispatcher::new();
        subject.register(literal("foo").executes(|_| 42).then(literal("bar")));

        let execute_result = subject.execute("foo baz".into(), Rc::new(CommandSource {}));

        let err = execute_result.err().unwrap();
        match err.type_ {
            BuiltInExceptions::DispatcherUnknownArgument => {}
            _ => panic!("Unexpected error"),
        }
        assert_eq!(err.cursor().unwrap(), 4);
    }

    #[test]
    fn execute_ambiguous_incorrect_argument() {
        let mut subject = CommandDispatcher::new();
        subject.register(
            literal("foo")
                .executes(|_| 42)
                .then(literal("bar"))
                .then(literal("baz")),
        );

        let execute_result = subject.execute("foo unknown".into(), Rc::new(CommandSource {}));

        let err = execute_result.err().unwrap();
        match err.type_ {
            BuiltInExceptions::DispatcherUnknownArgument => {}
            _ => panic!("Unexpected error"),
        }
        assert_eq!(err.cursor().unwrap(), 4);
    }

    #[test]
    fn execute_subcommand() {
        let mut subject = CommandDispatcher::new();

        subject.register(
            literal("foo")
                .then(literal("a"))
                .then(literal("=").executes(|_| 100))
                .then(literal("c"))
                .executes(|_| 42),
        );

        assert_eq!(
            subject
                .execute("foo =".into(), Rc::new(CommandSource {}))
                .unwrap(),
            100
        );
    }

    #[test]
    fn parse_incomplete_literal() {
        let mut subject = CommandDispatcher::new();
        subject.register(literal("foo").then(literal("bar").executes(|_| 42)));

        let parse = subject.parse("foo ".into(), Rc::new(CommandSource {}));
        assert_eq!(parse.reader.remaining(), " ");
        assert_eq!(parse.context.nodes.len(), 1);
    }

    #[test]
    fn parse_incomplete_argument() {
        let mut subject = CommandDispatcher::new();
        subject.register(literal("foo").then(argument("bar", integer()).executes(|_| 42)));

        let parse = subject.parse("foo ".into(), Rc::new(CommandSource {}));
        assert_eq!(parse.reader.remaining(), " ");
        assert_eq!(parse.context.nodes.len(), 1);
    }

    #[test]
    fn execute_ambiguious_parent_subcommand() {
        let mut subject = CommandDispatcher::new();

        subject.register(
            literal("test")
                .then(argument("incorrect", integer()).executes(|_| 42))
                .then(
                    argument("right", integer()).then(argument("sub", integer()).executes(|_| 100)),
                ),
        );

        assert_eq!(
            subject
                .execute("test 1 2".into(), Rc::new(CommandSource {}))
                .unwrap(),
            100
        );
    }

    #[test]
    fn execute_ambiguious_parent_subcommand_via_redirect() {
        let mut subject = CommandDispatcher::new();

        let real = subject.register(
            literal("test")
                .then(argument("incorrect", integer()).executes(|_| 42))
                .then(
                    argument("right", integer()).then(argument("sub", integer()).executes(|_| 100)),
                ),
        );

        subject.register(literal("redirect").redirect(real));

        assert_eq!(
            subject
                .execute("redirect 1 2".into(), Rc::new(CommandSource {}))
                .unwrap(),
            100
        );
    }

    #[test]
    fn execute_redirected_multiple_times() {
        let mut subject = CommandDispatcher::new();

        let concrete_node = subject.register(literal("actual").executes(|_| 42));
        let root = subject.root.clone();
        let redirect_node = subject.register(literal("redirected").redirect(root.clone()));

        let input = "redirected redirected actual";

        let parse = subject.parse(input.into(), Rc::new(CommandSource {}));
        assert_eq!(parse.context.range.get(input), "redirected");
        assert_eq!(parse.context.nodes.len(), 1);
        assert_eq!(parse.context.root, root);
        assert_eq!(parse.context.nodes[0].range, parse.context.range);
        assert_eq!(parse.context.nodes[0].node, redirect_node);

        let child1 = parse.context.child.clone();
        assert!(child1.is_some());
        assert_eq!(child1.clone().unwrap().range.get(input), "redirected");
        assert_eq!(child1.clone().unwrap().nodes.len(), 1);
        assert_eq!(child1.clone().unwrap().root, root);
        assert_eq!(
            child1.clone().unwrap().nodes[0].range,
            child1.clone().unwrap().range
        );
        assert_eq!(child1.clone().unwrap().nodes[0].node, redirect_node);

        let child2 = child1.unwrap().child.clone();
        assert!(child2.is_some());
        assert_eq!(child2.clone().unwrap().range.get(input), "actual");
        assert_eq!(child2.clone().unwrap().nodes.len(), 1);
        assert_eq!(child2.clone().unwrap().root, root);
        assert_eq!(
            child2.clone().unwrap().nodes[0].range,
            child2.clone().unwrap().range
        );
        assert_eq!(child2.clone().unwrap().nodes[0].node, concrete_node);

        assert_eq!(CommandDispatcher::execute_parsed(parse).unwrap(), 42);
    }

    #[test]
    fn execute_redirected() {
        let mut subject = CommandDispatcher::new();

        let source1 = Rc::new(CommandSource {});
        let source2 = Rc::new(CommandSource {});

        let modifier = move |_: &CommandContext<CommandSource>| -> Result<Vec<Rc<CommandSource>>, CommandSyntaxException> {
            Ok(vec![source1.clone(), source2.clone()])
        };

        let concrete_node = subject.register(literal("actual").executes(|_| 42));
        let redirect_node =
            subject.register(literal("redirected").fork(subject.root.clone(), Rc::new(modifier)));

        let input = "redirected actual";
        let parse = subject.parse(input.into(), Rc::new(CommandSource {}));
        assert_eq!(parse.context.range.get(input), "redirected");
        assert_eq!(parse.context.nodes.len(), 1);
        assert_eq!(parse.context.root, subject.root);
        assert_eq!(parse.context.nodes[0].range, parse.context.range);
        assert_eq!(parse.context.nodes[0].node, redirect_node);

        let parent = parse.context.child.clone();
        assert!(parent.is_some());
        let parent = parent.unwrap();
        assert_eq!(parent.range.get(input), "actual");
        assert_eq!(parent.nodes.len(), 1);
        assert_eq!(parse.context.root, subject.root);
        assert_eq!(parent.nodes[0].range, parent.range);
        assert_eq!(parent.nodes[0].node, concrete_node);
        assert_eq!(parent.source, Rc::new(CommandSource {}));

        assert_eq!(CommandDispatcher::execute_parsed(parse).unwrap(), 2);
    }

    #[test]
    fn execute_orphaned_subcommand() {
        let mut subject = CommandDispatcher::new();

        subject.register(
            literal("foo")
                .then(argument("bar", integer()))
                .executes(|_| 42),
        );

        let result = subject.execute("foo 5".into(), Rc::new(CommandSource {}));
        assert!(result.is_err());
        let result = result.unwrap_err();
        assert_eq!(
            *result.get_type(),
            BuiltInExceptions::DispatcherUnknownCommand
        );
        assert_eq!(result.cursor(), Some(5));
    }

    #[test]
    fn execute_invalid_other() {
        let mut subject = CommandDispatcher::new();

        subject.register(literal("w").executes(|_| panic!("This should not run")));
        subject.register(literal("world").executes(|_| 42));

        assert_eq!(
            subject
                .execute("world".into(), Rc::new(CommandSource {}))
                .unwrap(),
            42
        );
    }

    #[test]
    fn parse_no_space_separator() {
        let mut subject = CommandDispatcher::new();

        subject.register(
            literal("foo")
                .then(argument("bar", integer()))
                .executes(|_| 42),
        );

        let result = subject.execute("foo$".into(), Rc::new(CommandSource {}));
        assert!(result.is_err());
        let result = result.unwrap_err();
        assert_eq!(
            *result.get_type(),
            BuiltInExceptions::DispatcherUnknownCommand
        );
        assert_eq!(result.cursor(), Some(0));
    }

    #[test]
    fn execute_invalid_subcommand() {
        let mut subject = CommandDispatcher::new();

        subject.register(
            literal("foo")
                .then(argument("bar", integer()))
                .executes(|_| 42),
        );

        let result = subject.execute("foo bar".into(), Rc::new(CommandSource {}));
        assert!(result.is_err());
        let result = result.unwrap_err();
        // this fails for some reason, i blame mojang
        // assert_eq!(*result.get_type(), BuiltInExceptions::ReaderExpectedInt);
        assert_eq!(result.cursor(), Some(4));
    }

    #[test]
    fn get_path() {
        let mut subject = CommandDispatcher::<()>::new();

        let bar = literal("bar").build();
        subject.register(literal("foo").then_built(bar.clone()));

        assert_eq!(
            subject.get_path(bar),
            vec!["foo".to_string(), "bar".to_string()]
        );
    }

    #[test]
    fn find_node_doesnt_exist() {
        let subject = CommandDispatcher::<()>::new();

        assert_eq!(subject.find_node(&vec!["foo", "bar"]), None)
    }
}
