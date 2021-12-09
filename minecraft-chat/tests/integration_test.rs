use minecraft_chat::component::Component;
use serde_json::{Result, Value};

#[test]
fn test() {
    let j: Value = serde_json::from_str(
        r#"{
        "text": "hello",
        "color": "red",
        "bold": true
    }"#,
    )
    .unwrap();
    let component = Component::new(&j).unwrap();
    assert_eq!(component.to_ansi(None), "\x1b[38;2;255;85;85mhello\x1b[m");
}
