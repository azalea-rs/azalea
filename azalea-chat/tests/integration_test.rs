use azalea_chat::{
    FormattedText,
    style::{Ansi, ChatFormatting, TextColor},
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
    let component = FormattedText::deserialize(&j).unwrap();
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
        "obfuscated": true,
        "strikethrough": false
    },
    {
        "text": "!",
        "bold": true
    }
]"##,
    )
    .unwrap();
    let component = FormattedText::deserialize(&j).unwrap();

    assert_eq!(
        component.to_ansi(),
        format!(
            "{bold}{italic}{underlined}{red}hello{reset}{bold}{italic}{red} {reset}{italic}{underlined}{strikethrough}{abcdef}world{reset}{bold}{underlined}{obfuscated}{red} asdf{reset}{bold}{italic}{underlined}{red}!{reset}",
            bold = Ansi::BOLD,
            italic = Ansi::ITALIC,
            underlined = Ansi::UNDERLINED,
            red = Ansi::rgb(ChatFormatting::Red.color().unwrap()),
            reset = Ansi::RESET,
            strikethrough = Ansi::STRIKETHROUGH,
            obfuscated = Ansi::OBFUSCATED,
            abcdef = Ansi::rgb(TextColor::parse("#abcdef").unwrap().value),
        )
    );
}

#[test]
fn component_from_string() {
    let j: Value = serde_json::from_str("\"foo\"").unwrap();
    let component = FormattedText::deserialize(&j).unwrap();
    assert_eq!(component.to_ansi(), "\u{1b}[38;2;255;255;255mfoo\u{1b}[m");
}
