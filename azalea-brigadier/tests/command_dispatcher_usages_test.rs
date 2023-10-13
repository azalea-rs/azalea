use std::{collections::HashSet, sync::Arc};

use azalea_brigadier::{prelude::*, string_reader::StringReader, tree::CommandNode};
use parking_lot::RwLock;

fn setup() -> CommandDispatcher<()> {
    let command = |_: &CommandContext<()>| 0;

    let mut subject = CommandDispatcher::new();
    subject.register(
        literal("a")
            .then(
                literal("1")
                    .then(literal("i").executes(command))
                    .then(literal("ii").executes(command)),
            )
            .then(
                literal("2")
                    .then(literal("i").executes(command))
                    .then(literal("ii").executes(command)),
            ),
    );
    subject.register(literal("b").then(literal("1").executes(command)));
    subject.register(literal("c").executes(command));
    subject.register(literal("d").requires(|_| false).executes(command));
    subject.register(
        literal("e").executes(command).then(
            literal("1")
                .executes(command)
                .then(literal("i").executes(command))
                .then(literal("ii").executes(command)),
        ),
    );
    subject.register(
        literal("f")
            .then(
                literal("1")
                    .then(literal("i").executes(command))
                    .then(literal("ii").executes(command).requires(|_| false)),
            )
            .then(
                literal("2")
                    .then(literal("i").executes(command).requires(|_| false))
                    .then(literal("ii").executes(command)),
            ),
    );
    subject.register(
        literal("g")
            .executes(command)
            .then(literal("1").then(literal("i").executes(command))),
    );
    subject.register(
        literal("h")
            .executes(command)
            .then(literal("1").then(literal("i").executes(command)))
            .then(literal("2").then(literal("i").then(literal("ii").executes(command))))
            .then(literal("3").executes(command)),
    );
    subject.register(
        literal("i")
            .executes(command)
            .then(literal("1").executes(command))
            .then(literal("2").executes(command)),
    );
    subject.register(literal("j").redirect(subject.root.clone()));
    subject.register(literal("k").redirect(get(&subject, "h")));
    subject
}

fn get(subject: &CommandDispatcher<()>, command: &str) -> Arc<RwLock<CommandNode<()>>> {
    subject
        .parse(command.into(), ())
        .context
        .nodes
        .last()
        .unwrap()
        .node
        .clone()
}

#[test]
fn test_all_usage_no_commands() {
    let subject = CommandDispatcher::<()>::new();
    let results = subject.get_all_usage(&subject.root.read(), &(), true);
    assert!(results.is_empty());
}

#[test]
fn test_smart_usage_no_commands() {
    let subject = CommandDispatcher::<()>::new();
    let results = subject.get_smart_usage(&subject.root.read(), &());
    assert!(results.is_empty());
}

#[test]
fn test_all_usage_root() {
    let subject = setup();
    let results = subject.get_all_usage(&subject.root.read(), &(), true);

    let actual = results.into_iter().collect::<HashSet<_>>();
    let expected = vec![
        "a 1 i", "a 1 ii", "a 2 i", "a 2 ii", "b 1", "c", "e", "e 1", "e 1 i", "e 1 ii", "f 1 i",
        "f 2 ii", "g", "g 1 i", "h", "h 1 i", "h 2 i ii", "h 3", "i", "i 1", "i 2", "j ...",
        "k -> h",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect::<HashSet<_>>();
    assert_eq!(expected, actual);
}

#[test]
fn test_smart_usage_root() {
    let subject = setup();
    let results = subject.get_smart_usage(&subject.root.read(), &());

    let actual = results
        .into_iter()
        .map(|(k, v)| (k.read().name().to_owned(), v))
        .collect::<HashSet<_>>();

    let expected = vec![
        (get(&subject, "a"), "a (1|2)"),
        (get(&subject, "b"), "b 1"),
        (get(&subject, "c"), "c"),
        (get(&subject, "e"), "e [1]"),
        (get(&subject, "f"), "f (1|2)"),
        (get(&subject, "g"), "g [1]"),
        (get(&subject, "h"), "h [1|2|3]"),
        (get(&subject, "i"), "i [1|2]"),
        (get(&subject, "j"), "j ..."),
        (get(&subject, "k"), "k -> h"),
    ];

    println!("-");

    let expected = expected
        .into_iter()
        .map(|(k, v)| (k.read().name().to_owned(), v.to_owned()))
        .collect::<HashSet<_>>();

    assert_eq!(actual, expected);
}

#[test]
fn test_smart_usage_h() {
    let subject = setup();
    let results = subject.get_smart_usage(&get(&subject, "h").read(), &());

    let actual = results
        .into_iter()
        .map(|(k, v)| (k.read().name().to_owned(), v))
        .collect::<HashSet<_>>();

    let expected = vec![
        (get(&subject, "h 1"), "[1] i"),
        (get(&subject, "h 2"), "[2] i ii"),
        (get(&subject, "h 3"), "[3]"),
    ];

    let expected = expected
        .into_iter()
        .map(|(k, v)| (k.read().name().to_owned(), v.to_owned()))
        .collect::<HashSet<_>>();

    assert_eq!(actual, expected);
}

#[test]
fn test_smart_usage_offset_h() {
    let subject = setup();
    let mut offset_h = StringReader::from("/|/|/h");
    offset_h.cursor = 5;

    let results = subject.get_smart_usage(&get(&subject, "h").read(), &());

    let actual = results
        .into_iter()
        .map(|(k, v)| (k.read().name().to_owned(), v))
        .collect::<HashSet<_>>();

    let expected = vec![
        (get(&subject, "h 1"), "[1] i"),
        (get(&subject, "h 2"), "[2] i ii"),
        (get(&subject, "h 3"), "[3]"),
    ];

    let expected = expected
        .into_iter()
        .map(|(k, v)| (k.read().name().to_owned(), v.to_owned()))
        .collect::<HashSet<_>>();

    assert_eq!(actual, expected);
}
