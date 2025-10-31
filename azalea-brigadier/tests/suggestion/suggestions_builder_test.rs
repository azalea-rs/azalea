use std::collections::HashSet;

use azalea_brigadier::{
    context::StringRange,
    suggestion::{Suggestion, SuggestionsBuilder},
};

#[test]
fn suggest_appends() {
    let builder = SuggestionsBuilder::new("Hello w", 6);
    let result = builder.suggest("orld!").build();
    assert_eq!(
        result.list(),
        vec![Suggestion::new(StringRange::between(6, 7), "orld!")]
    );
    assert_eq!(result.range(), StringRange::between(6, 7));
    assert!(!result.is_empty());
}

#[test]
fn suggest_replaces() {
    let builder = SuggestionsBuilder::new("Hello w", 6);
    let result = builder.suggest("everybody").build();
    assert_eq!(
        result.list(),
        vec![Suggestion::new(StringRange::between(6, 7), "everybody")]
    );
    assert_eq!(result.range(), StringRange::between(6, 7));
    assert!(!result.is_empty());
}

#[test]
fn suggest_noop() {
    let builder = SuggestionsBuilder::new("Hello w", 6);
    let result = builder.suggest("w").build();
    assert_eq!(result.list(), vec![]);
    assert!(result.is_empty());
}

#[test]
fn suggest_multiple() {
    let builder = SuggestionsBuilder::new("Hello w", 6);
    let result = builder
        .suggest("world!")
        .suggest("everybody")
        .suggest("weekend")
        .build();
    assert_eq!(
        result.list(),
        vec![
            Suggestion::new(StringRange::between(6, 7), "everybody"),
            Suggestion::new(StringRange::between(6, 7), "weekend"),
            Suggestion::new(StringRange::between(6, 7), "world!"),
        ]
    );
    assert_eq!(result.range(), StringRange::between(6, 7));
    assert!(!result.is_empty());
}

#[test]
fn restart() {
    let builder = SuggestionsBuilder::new("Hello w", 6);
    let builder = builder.suggest("won't be included in restart");
    let other = builder.restart();
    assert_ne!(other, builder);
    assert_eq!(other.input(), builder.input());
    assert_eq!(other.start(), builder.start());
    assert_eq!(other.remaining(), builder.remaining());
}

#[test]
fn sort_alphabetical() {
    let builder = SuggestionsBuilder::new("Hello w", 6);
    let result = builder
        .suggest("2")
        .suggest("4")
        .suggest("6")
        .suggest("8")
        .suggest("30")
        .suggest("32")
        .build();
    let actual = result.list().iter().map(|s| s.text()).collect::<Vec<_>>();
    assert_eq!(actual, vec!["2", "30", "32", "4", "6", "8"]);
}

#[test]
fn sort_numerical() {
    let builder = SuggestionsBuilder::new("Hello w", 6);
    let result = builder
        .suggest_integer(2)
        .suggest_integer(4)
        .suggest_integer(6)
        .suggest_integer(8)
        .suggest_integer(30)
        .suggest_integer(32)
        .build();
    let actual = result.list().iter().map(|s| s.text()).collect::<Vec<_>>();
    assert_eq!(actual, vec!["2", "4", "6", "8", "30", "32"]);
}

#[test]
fn sort_mixed() {
    let builder = SuggestionsBuilder::new("Hello w", 6);
    let result = builder
        .suggest("11")
        .suggest("22")
        .suggest("33")
        .suggest("a")
        .suggest("b")
        .suggest("c")
        .suggest_integer(2)
        .suggest_integer(4)
        .suggest_integer(6)
        .suggest_integer(8)
        .suggest_integer(30)
        .suggest_integer(32)
        .suggest("3a")
        .suggest("a3")
        .build();
    let actual = result
        .list()
        .iter()
        .map(|s| s.text())
        .collect::<HashSet<_>>();
    // mojang please
    let expected = vec![
        "11", "2", "22", "33", "3a", "4", "6", "8", "30", "32", "a", "a3", "b", "c",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect::<HashSet<_>>();
    assert_eq!(actual, expected);
}
