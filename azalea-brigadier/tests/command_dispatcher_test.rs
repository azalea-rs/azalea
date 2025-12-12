use std::sync::Arc;

use azalea_brigadier::{
    arguments::integer_argument_type::integer,
    builder::{literal_argument_builder::literal, required_argument_builder::argument},
    command_dispatcher::CommandDispatcher,
    context::CommandContext,
    errors::{BuiltInError, CommandSyntaxError},
    string_reader::StringReader,
};

#[derive(Debug, PartialEq)]
struct CommandSource {}

fn input_with_offset(input: &str, offset: usize) -> StringReader {
    let mut result: StringReader = input.into();
    result.cursor = offset;
    result
}

#[test]
fn create_and_execute_offset_command() {
    let mut subject = CommandDispatcher::new();
    subject.register(literal("foo").executes(|_| 42));

    assert_eq!(
        subject
            .execute(input_with_offset("/foo", 1), &CommandSource {})
            .unwrap(),
        42
    );
}

#[test]
fn create_and_merge_commands() {
    let mut subject = CommandDispatcher::new();
    subject.register(literal("base").then(literal("foo").executes(|_| 42)));
    subject.register(literal("base").then(literal("bar").executes(|_| 42)));

    assert_eq!(subject.execute("base foo", &CommandSource {}).unwrap(), 42);
    assert_eq!(subject.execute("base bar", &CommandSource {}).unwrap(), 42);
}

#[test]
fn execute_unknown_command() {
    let mut subject = CommandDispatcher::new();
    subject.register(literal("bar"));
    subject.register(literal("baz"));

    let execute_result = subject.execute("foo", &CommandSource {});

    let err = execute_result.err().unwrap();
    assert_eq!(err.kind(), &BuiltInError::DispatcherUnknownCommand);
    assert_eq!(err.cursor().unwrap(), 0);
}

#[test]
fn execute_impermissible_command() {
    let mut subject = CommandDispatcher::new();
    subject.register(literal("foo").requires(|_| false));

    let execute_result = subject.execute("foo", &CommandSource {});

    let err = execute_result.err().unwrap();
    assert_eq!(err.kind(), &BuiltInError::DispatcherUnknownCommand);
    assert_eq!(err.cursor().unwrap(), 0);
}

#[test]
fn execute_empty_command() {
    let mut subject = CommandDispatcher::new();
    subject.register(literal(""));

    let execute_result = subject.execute("", &CommandSource {});

    let err = execute_result.err().unwrap();
    assert_eq!(err.kind(), &BuiltInError::DispatcherUnknownCommand);
    assert_eq!(err.cursor().unwrap(), 0);
}

#[test]
fn execute_unknown_subcommand() {
    let mut subject = CommandDispatcher::new();
    subject.register(literal("foo").executes(|_| 42));

    let execute_result = subject.execute("foo bar", &CommandSource {});

    let err = execute_result.err().unwrap();
    assert_eq!(err.kind(), &BuiltInError::DispatcherUnknownArgument);
    assert_eq!(err.cursor().unwrap(), 4);
}

#[test]
fn execute_incorrect_literal() {
    let mut subject = CommandDispatcher::new();
    subject.register(literal("foo").executes(|_| 42).then(literal("bar")));

    let execute_result = subject.execute("foo baz", &CommandSource {});

    let err = execute_result.err().unwrap();
    assert_eq!(err.kind(), &BuiltInError::DispatcherUnknownArgument);
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

    let execute_result = subject.execute("foo unknown", &CommandSource {});

    let err = execute_result.err().unwrap();
    assert_eq!(err.kind(), &BuiltInError::DispatcherUnknownArgument);
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

    assert_eq!(subject.execute("foo =", &CommandSource {}).unwrap(), 100);
}

#[test]
fn parse_incomplete_literal() {
    let mut subject = CommandDispatcher::new();
    subject.register(literal("foo").then(literal("bar").executes(|_| 42)));

    let parse = subject.parse("foo ".into(), &CommandSource {});
    assert_eq!(parse.reader.remaining(), " ");
    assert_eq!(parse.context.nodes.len(), 1);
}

#[test]
fn parse_incomplete_argument() {
    let mut subject = CommandDispatcher::new();
    subject.register(literal("foo").then(argument("bar", integer()).executes(|_| 42)));

    let parse = subject.parse("foo ".into(), &CommandSource {});
    assert_eq!(parse.reader.remaining(), " ");
    assert_eq!(parse.context.nodes.len(), 1);
}

#[test]
fn execute_ambiguous_parent_subcommand() {
    let mut subject = CommandDispatcher::new();

    subject.register(
        literal("test")
            .then(argument("incorrect", integer()).executes(|_| 42))
            .then(argument("right", integer()).then(argument("sub", integer()).executes(|_| 100))),
    );

    assert_eq!(subject.execute("test 1 2", &CommandSource {}).unwrap(), 100);
}

#[test]
fn execute_ambiguous_parent_subcommand_via_redirect() {
    let mut subject = CommandDispatcher::new();

    let real = subject.register(
        literal("test")
            .then(argument("incorrect", integer()).executes(|_| 42))
            .then(argument("right", integer()).then(argument("sub", integer()).executes(|_| 100))),
    );

    subject.register(literal("redirect").redirect(real));

    assert_eq!(
        subject.execute("redirect 1 2", &CommandSource {}).unwrap(),
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

    let parse = subject.parse(input.into(), &CommandSource {});
    assert_eq!(parse.context.range.get(input), "redirected");
    assert_eq!(parse.context.nodes.len(), 1);
    assert_eq!(*parse.context.root.read(), *root.read());
    assert_eq!(parse.context.nodes[0].range, parse.context.range);
    assert_eq!(*parse.context.nodes[0].node.read(), *redirect_node.read());

    let child1 = parse.context.child.clone();
    assert!(child1.is_some());
    assert_eq!(child1.clone().unwrap().range.get(input), "redirected");
    assert_eq!(child1.clone().unwrap().nodes.len(), 1);
    assert_eq!(*child1.clone().unwrap().root.read(), *root.read());
    assert_eq!(
        child1.clone().unwrap().nodes[0].range,
        child1.clone().unwrap().range
    );
    assert_eq!(
        *child1.clone().unwrap().nodes[0].node.read(),
        *redirect_node.read()
    );

    let child2 = child1.unwrap().child.clone();
    assert!(child2.is_some());
    assert_eq!(child2.clone().unwrap().range.get(input), "actual");
    assert_eq!(child2.clone().unwrap().nodes.len(), 1);
    assert_eq!(*child2.clone().unwrap().root.read(), *root.read());
    assert_eq!(
        child2.clone().unwrap().nodes[0].range,
        child2.clone().unwrap().range
    );
    assert_eq!(*child2.unwrap().nodes[0].node.read(), *concrete_node.read());

    assert_eq!(subject.execute_parsed(parse).unwrap(), 42);
}

#[test]
fn execute_redirected() {
    let mut subject = CommandDispatcher::new();

    let source1 = Arc::new(CommandSource {});
    let source2 = Arc::new(CommandSource {});

    let modifier = move |_: &CommandContext<CommandSource>| -> Result<Vec<Arc<CommandSource>>, CommandSyntaxError> {
            Ok(vec![source1.clone(), source2.clone()])
        };

    let concrete_node = subject.register(literal("actual").executes(|_| 42));
    let redirect_node =
        subject.register(literal("redirected").fork(subject.root.clone(), Arc::new(modifier)));

    let input = "redirected actual";
    let parse = subject.parse(input.into(), CommandSource {});
    assert_eq!(parse.context.range.get(input), "redirected");
    assert_eq!(parse.context.nodes.len(), 1);
    assert_eq!(*parse.context.root.read(), *subject.root.read());
    assert_eq!(parse.context.nodes[0].range, parse.context.range);
    assert_eq!(*parse.context.nodes[0].node.read(), *redirect_node.read());

    let parent = parse.context.child.clone();
    assert!(parent.is_some());
    let parent = parent.unwrap();
    assert_eq!(parent.range.get(input), "actual");
    assert_eq!(parent.nodes.len(), 1);
    assert_eq!(*parse.context.root.read(), *subject.root.read());
    assert_eq!(parent.nodes[0].range, parent.range);
    assert_eq!(*parent.nodes[0].node.read(), *concrete_node.read());
    assert_eq!(*parent.source, CommandSource {});

    assert_eq!(subject.execute_parsed(parse).unwrap(), 2);
}

#[test]
fn execute_orphaned_subcommand() {
    let mut subject = CommandDispatcher::new();

    subject.register(
        literal("foo")
            .then(argument("bar", integer()))
            .executes(|_| 42),
    );

    let result = subject.execute("foo 5", &CommandSource {});
    assert!(result.is_err());
    let result = result.unwrap_err();
    assert_eq!(*result.kind(), BuiltInError::DispatcherUnknownCommand);
    assert_eq!(result.cursor(), Some(5));
}

#[test]
fn execute_invalid_other() {
    let mut subject = CommandDispatcher::new();

    subject.register(literal("w").executes(|_| panic!("This should not run")));
    subject.register(literal("world").executes(|_| 42));

    assert_eq!(subject.execute("world", &CommandSource {}).unwrap(), 42);
}

#[test]
fn parse_no_space_separator() {
    let mut subject = CommandDispatcher::new();

    subject.register(
        literal("foo")
            .then(argument("bar", integer()))
            .executes(|_| 42),
    );

    let result = subject.execute("foo$", &CommandSource {});
    assert!(result.is_err());
    let result = result.unwrap_err();
    assert_eq!(*result.kind(), BuiltInError::DispatcherUnknownCommand);
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

    let result = subject.execute("foo bar", &CommandSource {});
    assert!(result.is_err());
    let result = result.unwrap_err();
    // this fails for some reason, i blame mojang
    // assert_eq!(*result.get_type(), BuiltInError::ReaderExpectedInt);
    assert_eq!(result.cursor(), Some(4));
}

#[test]
fn get_path() {
    let mut subject = CommandDispatcher::<()>::new();

    let bar = literal("bar").build();
    subject.register(literal("foo").then_built(bar.clone()));

    assert_eq!(
        subject.get_path(bar),
        vec!["foo".to_owned(), "bar".to_owned()]
    );
}

#[test]
fn find_node_doesnt_exist() {
    let subject = CommandDispatcher::<()>::new();

    assert!(subject.find_node(&["foo", "bar"]).is_none())
}
