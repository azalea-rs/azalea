use std::collections::HashSet;

use azalea_brigadier::{
    context::StringRange,
    suggestion::{Suggestion, Suggestions},
};

#[test]
fn merge_empty() {
    let merged = Suggestions::merge("foo b", &[]);
    assert!(merged.is_empty());
}

#[test]
fn merge_single() {
    let suggestions = Suggestions::new(
        StringRange::at(5),
        vec![Suggestion::new(StringRange::at(5), "ar")],
    );
    let merged = Suggestions::merge("foo b", std::slice::from_ref(&suggestions));
    assert_eq!(merged, suggestions);
}

#[test]
fn merge_multiple() {
    let a = Suggestions::new(
        StringRange::at(5),
        vec![
            Suggestion::new(StringRange::at(5), "ar"),
            Suggestion::new(StringRange::at(5), "az"),
            Suggestion::new(StringRange::at(5), "Az"),
        ],
    );
    let b = Suggestions::new(
        StringRange::between(4, 5),
        vec![
            Suggestion::new(StringRange::between(4, 5), "foo"),
            Suggestion::new(StringRange::between(4, 5), "qux"),
            Suggestion::new(StringRange::between(4, 5), "apple"),
            Suggestion::new(StringRange::between(4, 5), "Bar"),
        ],
    );
    let merged = Suggestions::merge("foo b", &[a, b]);

    let actual = merged.list().iter().cloned().collect::<HashSet<_>>();
    let expected = vec![
        Suggestion::new(StringRange::between(4, 5), "apple"),
        Suggestion::new(StringRange::between(4, 5), "bar"),
        Suggestion::new(StringRange::between(4, 5), "Bar"),
        Suggestion::new(StringRange::between(4, 5), "baz"),
        Suggestion::new(StringRange::between(4, 5), "bAz"),
        Suggestion::new(StringRange::between(4, 5), "foo"),
        Suggestion::new(StringRange::between(4, 5), "qux"),
    ]
    .into_iter()
    .collect::<HashSet<_>>();
    assert_eq!(actual, expected);
}
