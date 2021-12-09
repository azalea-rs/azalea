use minecraft_chat::{
    component::Component,
    style::{Ansi, ChatFormatting, TextColor},
};
use serde_json::{Result, Value};

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
    let component = Component::new(&j).unwrap();
    assert_eq!(
        component.to_ansi(None),
        "\x1b[1m\x1b[38;2;255;85;85mhello\x1b[m"
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
    let component = Component::new(&j).unwrap();
    assert_eq!(
        component.to_ansi(None),
        format!(
            "{bold}{italic}{underlined}{red}hello{reset}{bold}{italic}{red} {reset}{italic}{strikethrough}{abcdef}world{reset}{abcdef} asdf{bold}!{reset}",
            bold = Ansi::BOLD,
            italic = Ansi::ITALIC,
            underlined = Ansi::UNDERLINED,
            red = Ansi::rgb(ChatFormatting::RED.color.unwrap()),
            reset = Ansi::RESET,
            strikethrough = Ansi::STRIKETHROUGH,
            abcdef = Ansi::rgb(TextColor::parse("#abcdef".to_string()).unwrap().value),
        )
    );
}
