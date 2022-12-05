use azalea_chat::{
    style::{Ansi, ChatFormatting, TextColor},
    Component,
};
use serde::Deserialize;
use serde_json::Value;

#[test]
fn basic_ansi_test() {
    let j: Value = serde_json::from_str(
        r#"{
    "text": "hello",
    "color": "red",
    "bold": true
}"#,
    )
    .unwrap();
    let component = Component::deserialize(&j).unwrap();
    assert_eq!(
        component.to_ansi(),
        "\u{1b}[1m\u{1b}[38;2;255;85;85mhello\u{1b}[m"
    );
}

#[test]
fn complex_ansi_test() {
    let j: Value = serde_json::from_str(
        r##"[
    {
        "text": "hello",
        "color": "red",
        "bold": true,
        "italic": true,
        "underlined": true,
        "adsfsf": "this should be ignored",
        "extra": [
            {"text": " ", "underlined": false},
            {"text": "world", "bold": false, "strikethrough": true, "color": "#abcdef"}
        ]
    },
    {
        "text": " asdf",
        "italic": false,
        "obfuscated": "true",
        "strikethrough": false
    },
    {
        "text": "!",
        "bold": true
    }
]"##,
    )
    .unwrap();
    let component = Component::deserialize(&j).unwrap();
    assert_eq!(
        component.to_ansi(),
        format!(
            "{bold}{italic}{underlined}{red}hello{reset}{bold}{italic}{red} {reset}{italic}{strikethrough}{abcdef}world{reset}{abcdef} asdf{bold}!{reset}",
            bold = Ansi::BOLD,
            italic = Ansi::ITALIC,
            underlined = Ansi::UNDERLINED,
            red = Ansi::rgb(ChatFormatting::Red.color().unwrap()),
            reset = Ansi::RESET,
            strikethrough = Ansi::STRIKETHROUGH,
            abcdef = Ansi::rgb(TextColor::parse("#abcdef".to_string()).unwrap().value),
        )
    );
}

#[test]
fn component_from_string() {
    let j: Value = serde_json::from_str("\"foo\"").unwrap();
    let component = Component::deserialize(&j).unwrap();
    assert_eq!(component.to_ansi(), "foo");
}
