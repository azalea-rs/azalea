use azalea_brigadier::{context::StringRange, suggestion::Suggestion};

#[test]
fn apply_insertation_start() {
    let suggestion = Suggestion::new(StringRange::at(0), "And so I said: ");
    assert_eq!(
        suggestion.apply("Hello world!"),
        "And so I said: Hello world!"
    );
}

#[test]
fn apply_insertation_middle() {
    let suggestion = Suggestion::new(StringRange::at(6), "small ");
    assert_eq!(suggestion.apply("Hello world!"), "Hello small world!");
}

#[test]
fn apply_insertation_end() {
    let suggestion = Suggestion::new(StringRange::at(5), " world!");
    assert_eq!(suggestion.apply("Hello"), "Hello world!");
}

#[test]
fn apply_replacement_start() {
    let suggestion = Suggestion::new(StringRange::between(0, 5), "Goodbye");
    assert_eq!(suggestion.apply("Hello world!"), "Goodbye world!");
}

#[test]
fn apply_replacement_middle() {
    let suggestion = Suggestion::new(StringRange::between(6, 11), "Alex");
    assert_eq!(suggestion.apply("Hello world!"), "Hello Alex!");
}

#[test]
fn apply_replacement_end() {
    let suggestion = Suggestion::new(StringRange::between(6, 12), "Creeper!");
    assert_eq!(suggestion.apply("Hello world!"), "Hello Creeper!");
}

#[test]
fn apply_replacement_everything() {
    let suggestion = Suggestion::new(StringRange::between(0, 12), "Oh dear.");
    assert_eq!(suggestion.apply("Hello world!"), "Oh dear.");
}

#[test]
fn expand_unchanged() {
    let suggestion = Suggestion::new(StringRange::at(1), "oo");
    assert_eq!(suggestion.expand("f", StringRange::at(1)), suggestion);
}

#[test]
fn expand_left() {
    let suggestion = Suggestion::new(StringRange::at(1), "oo");
    assert_eq!(
        suggestion.expand("f", StringRange::between(0, 1)),
        Suggestion::new(StringRange::between(0, 1), "foo")
    );
}

#[test]
fn expand_right() {
    let suggestion = Suggestion::new(StringRange::at(0), "minecraft:");
    assert_eq!(
        suggestion.expand("fish", StringRange::between(0, 4)),
        Suggestion::new(StringRange::between(0, 4), "minecraft:fish")
    );
}

#[test]
fn expand_both() {
    let suggestion = Suggestion::new(StringRange::at(11), "minecraft:");
    assert_eq!(
        suggestion.expand("give Steve fish_block", StringRange::between(5, 21)),
        Suggestion::new(StringRange::between(5, 21), "Steve minecraft:fish_block")
    );
}

#[test]
fn expand_replacement() {
    let suggestion = Suggestion::new(StringRange::between(6, 11), "strangers");
    assert_eq!(
        suggestion.expand("Hello world!", StringRange::between(0, 12)),
        Suggestion::new(StringRange::between(0, 12), "Hello strangers!")
    );
}
