use azalea_brigadier::{
    context::StringRange, prelude::*, string_reader::StringReader, suggestion::Suggestion,
};

fn test_suggestions(
    subject: &CommandDispatcher<()>,
    contents: &str,
    cursor: usize,
    range: StringRange,
    suggestions: Vec<&str>,
) {
    let result = CommandDispatcher::get_completion_suggestions_with_cursor(
        subject.parse(contents.into(), ()),
        cursor,
    );
    assert_eq!(result.range(), range);

    let mut expected = Vec::new();
    for suggestion in suggestions {
        expected.push(Suggestion::new(range, suggestion));
    }

    assert_eq!(result.list(), expected);
}

fn input_with_offset(input: &str, offset: usize) -> StringReader {
    let mut result = StringReader::from(input);
    result.cursor = offset;
    result
}

#[test]
fn get_completion_suggestions_root_commands() {
    let mut subject = CommandDispatcher::<()>::new();
    subject.register(literal("foo"));
    subject.register(literal("bar"));
    subject.register(literal("baz"));

    let result = CommandDispatcher::get_completion_suggestions(subject.parse("".into(), ()));

    assert_eq!(result.range(), StringRange::at(0));
    assert_eq!(
        result.list(),
        vec![
            Suggestion::new(StringRange::at(0), "bar"),
            Suggestion::new(StringRange::at(0), "baz"),
            Suggestion::new(StringRange::at(0), "foo")
        ]
    );
}

#[test]
fn get_completion_suggestions_root_commands_with_input_offset() {
    let mut subject = CommandDispatcher::<()>::new();
    subject.register(literal("foo"));
    subject.register(literal("bar"));
    subject.register(literal("baz"));

    let result = CommandDispatcher::get_completion_suggestions(
        subject.parse(input_with_offset("OOO", 3), ()),
    );

    assert_eq!(result.range(), StringRange::at(3));
    assert_eq!(
        result.list(),
        vec![
            Suggestion::new(StringRange::at(3), "bar"),
            Suggestion::new(StringRange::at(3), "baz"),
            Suggestion::new(StringRange::at(3), "foo")
        ]
    );
}

#[test]
fn get_completion_suggestions_root_commands_partial() {
    let mut subject = CommandDispatcher::<()>::new();
    subject.register(literal("foo"));
    subject.register(literal("bar"));
    subject.register(literal("baz"));

    let result = CommandDispatcher::get_completion_suggestions(subject.parse("b".into(), ()));

    assert_eq!(result.range(), StringRange::between(0, 1));
    assert_eq!(
        result.list(),
        vec![
            Suggestion::new(StringRange::between(0, 1), "bar"),
            Suggestion::new(StringRange::between(0, 1), "baz")
        ]
    );
}

#[test]
fn get_completion_suggestions_root_commands_partial_with_input_offset() {
    let mut subject = CommandDispatcher::<()>::new();
    subject.register(literal("foo"));
    subject.register(literal("bar"));
    subject.register(literal("baz"));

    let result = CommandDispatcher::get_completion_suggestions(
        subject.parse(input_with_offset("Zb", 1), ()),
    );

    assert_eq!(result.range(), StringRange::between(1, 2));
    assert_eq!(
        result.list(),
        vec![
            Suggestion::new(StringRange::between(1, 2), "bar"),
            Suggestion::new(StringRange::between(1, 2), "baz")
        ]
    );
}

#[test]
fn get_completion_suggestions_sub_commands() {
    let mut subject = CommandDispatcher::<()>::new();
    subject.register(
        literal("parent")
            .then(literal("foo"))
            .then(literal("bar"))
            .then(literal("baz")),
    );

    let result = CommandDispatcher::get_completion_suggestions(subject.parse("parent ".into(), ()));

    assert_eq!(result.range(), StringRange::at(7));
    assert_eq!(
        result.list(),
        vec![
            Suggestion::new(StringRange::at(7), "bar"),
            Suggestion::new(StringRange::at(7), "baz"),
            Suggestion::new(StringRange::at(7), "foo")
        ]
    );
}

#[test]
fn get_completion_suggestions_moving_cursor_sub_commands() {
    let mut subject = CommandDispatcher::<()>::new();
    subject.register(
        literal("parent_one")
            .then(literal("faz"))
            .then(literal("fbz"))
            .then(literal("gaz")),
    );

    subject.register(literal("parent_two"));

    test_suggestions(
        &subject,
        "parent_one faz ",
        0,
        StringRange::at(0),
        vec!["parent_one", "parent_two"],
    );
    test_suggestions(
        &subject,
        "parent_one faz ",
        1,
        StringRange::between(0, 1),
        vec!["parent_one", "parent_two"],
    );
    test_suggestions(
        &subject,
        "parent_one faz ",
        7,
        StringRange::between(0, 7),
        vec!["parent_one", "parent_two"],
    );
    test_suggestions(
        &subject,
        "parent_one faz ",
        8,
        StringRange::between(0, 8),
        vec!["parent_one"],
    );
    test_suggestions(&subject, "parent_one faz ", 10, StringRange::at(0), vec![]);
    test_suggestions(
        &subject,
        "parent_one faz ",
        11,
        StringRange::at(11),
        vec!["faz", "fbz", "gaz"],
    );
    test_suggestions(
        &subject,
        "parent_one faz ",
        12,
        StringRange::between(11, 12),
        vec!["faz", "fbz"],
    );
    test_suggestions(
        &subject,
        "parent_one faz ",
        13,
        StringRange::between(11, 13),
        vec!["faz"],
    );
    test_suggestions(&subject, "parent_one faz ", 14, StringRange::at(0), vec![]);
    test_suggestions(&subject, "parent_one faz ", 15, StringRange::at(0), vec![]);
}

#[test]
fn get_completion_suggestions_sub_commands_partial() {
    let mut subject = CommandDispatcher::<()>::new();
    subject.register(
        literal("parent")
            .then(literal("foo"))
            .then(literal("bar"))
            .then(literal("baz")),
    );

    let parse = subject.parse("parent b".into(), ());

    let result = CommandDispatcher::get_completion_suggestions(parse);

    assert_eq!(result.range(), StringRange::between(7, 8));
    assert_eq!(
        result.list(),
        vec![
            Suggestion::new(StringRange::between(7, 8), "bar"),
            Suggestion::new(StringRange::between(7, 8), "baz")
        ]
    );
}

#[test]
fn get_completion_suggestions_sub_commands_partial_with_input_offset() {
    let mut subject = CommandDispatcher::<()>::new();
    subject.register(
        literal("parent")
            .then(literal("foo"))
            .then(literal("bar"))
            .then(literal("baz")),
    );

    let parse = subject.parse(input_with_offset("junk parent b", 5), ());

    let result = CommandDispatcher::get_completion_suggestions(parse);

    assert_eq!(result.range(), StringRange::between(12, 13));
    assert_eq!(
        result.list(),
        vec![
            Suggestion::new(StringRange::between(12, 13), "bar"),
            Suggestion::new(StringRange::between(12, 13), "baz")
        ]
    );
}

#[test]
fn get_completion_suggestions_redirect() {
    let mut subject = CommandDispatcher::<()>::new();
    let actual = subject.register(literal("actual").then(literal("sub")));
    subject.register(literal("redirect").redirect(actual));

    let parse = subject.parse("redirect ".into(), ());

    let result = CommandDispatcher::get_completion_suggestions(parse);

    assert_eq!(result.range(), StringRange::at(9));
    assert_eq!(
        result.list(),
        vec![Suggestion::new(StringRange::at(9), "sub")]
    );
}

#[test]
fn get_completion_suggestions_redirect_partial() {
    let mut subject = CommandDispatcher::<()>::new();
    let actual = subject.register(literal("actual").then(literal("sub")));
    subject.register(literal("redirect").redirect(actual));

    let parse = subject.parse("redirect s".into(), ());

    let result = CommandDispatcher::get_completion_suggestions(parse);

    assert_eq!(result.range(), StringRange::between(9, 10));
    assert_eq!(
        result.list(),
        vec![Suggestion::new(StringRange::between(9, 10), "sub")]
    );
}

#[test]
fn get_completion_suggestions_moving_cursor_redirect() {
    let mut subject = CommandDispatcher::<()>::new();
    let actual_one = subject.register(
        literal("actual_one")
            .then(literal("faz"))
            .then(literal("fbz"))
            .then(literal("gaz")),
    );

    subject.register(literal("actual_two"));

    subject.register(literal("redirect_one").redirect(actual_one.clone()));
    subject.register(literal("redirect_two").redirect(actual_one));

    test_suggestions(
        &subject,
        "redirect_one faz ",
        0,
        StringRange::at(0),
        vec!["actual_one", "actual_two", "redirect_one", "redirect_two"],
    );
    test_suggestions(
        &subject,
        "redirect_one faz ",
        9,
        StringRange::between(0, 9),
        vec!["redirect_one", "redirect_two"],
    );
    test_suggestions(
        &subject,
        "redirect_one faz ",
        10,
        StringRange::between(0, 10),
        vec!["redirect_one"],
    );
    test_suggestions(
        &subject,
        "redirect_one faz ",
        12,
        StringRange::at(0),
        vec![],
    );
    test_suggestions(
        &subject,
        "redirect_one faz ",
        13,
        StringRange::at(13),
        vec!["faz", "fbz", "gaz"],
    );
    test_suggestions(
        &subject,
        "redirect_one faz ",
        14,
        StringRange::between(13, 14),
        vec!["faz", "fbz"],
    );
    test_suggestions(
        &subject,
        "redirect_one faz ",
        15,
        StringRange::between(13, 15),
        vec!["faz"],
    );
    test_suggestions(
        &subject,
        "redirect_one faz ",
        16,
        StringRange::at(0),
        vec![],
    );
    test_suggestions(
        &subject,
        "redirect_one faz ",
        17,
        StringRange::at(0),
        vec![],
    );
}

#[test]
fn get_completion_suggestions_redirect_partial_with_input_offset() {
    let mut subject = CommandDispatcher::<()>::new();
    let actual = subject.register(literal("actual").then(literal("sub")));
    subject.register(literal("redirect").redirect(actual));

    let parse = subject.parse(input_with_offset("/redirect s", 1), ());

    let result = CommandDispatcher::get_completion_suggestions(parse);

    assert_eq!(result.range(), StringRange::between(10, 11));
    assert_eq!(
        result.list(),
        vec![Suggestion::new(StringRange::between(10, 11), "sub")]
    );
}

#[test]
fn get_completion_suggestions_redirect_lots() {
    let mut subject = CommandDispatcher::<()>::new();
    let loop_ = subject.register(literal("redirect"));
    subject.register(
        literal("redirect").then(literal("loop").then(argument("loop", integer()).redirect(loop_))),
    );

    let result = CommandDispatcher::get_completion_suggestions(
        subject.parse("redirect loop 1 loop 02 loop 003 ".into(), ()),
    );

    assert_eq!(result.range(), StringRange::at(33));
    assert_eq!(
        result.list(),
        vec![Suggestion::new(StringRange::at(33), "loop")]
    );
}

#[test]
fn get_completion_suggestions_execute_simulation() {
    let mut subject = CommandDispatcher::<()>::new();
    let execute = subject.register(literal("execute"));
    subject.register(
        literal("execute")
            .then(literal("as").then(argument("name", word()).redirect(execute.clone())))
            .then(literal("store").then(argument("name", word()).redirect(execute)))
            .then(literal("run").executes(|_| 0)),
    );

    let parse = subject.parse("execute as Dinnerbone as".into(), ());

    let result = CommandDispatcher::get_completion_suggestions(parse);

    assert!(result.is_empty());
}

#[test]
fn get_completion_suggestions_execute_simulation_partial() {
    let mut subject = CommandDispatcher::<()>::new();
    let execute = subject.register(literal("execute"));
    subject.register(
        literal("execute")
            .then(
                literal("as")
                    .then(literal("bar").redirect(execute.clone()))
                    .then(literal("baz").redirect(execute.clone())),
            )
            .then(literal("store").then(argument("name", word()).redirect(execute)))
            .then(literal("run").executes(|_| 0)),
    );

    let parse = subject.parse("execute as bar as ".into(), ());

    let result = CommandDispatcher::get_completion_suggestions(parse);

    assert_eq!(result.range(), StringRange::at(18));
    assert_eq!(
        result.list(),
        vec![
            Suggestion::new(StringRange::at(18), "bar"),
            Suggestion::new(StringRange::at(18), "baz")
        ]
    );
}
