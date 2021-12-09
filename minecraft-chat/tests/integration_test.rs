use minecraft_chat::component::Component;
use serde_json::{Result, Value};

#[test]
fn test() {
    let j: Value = serde_json::from_str(
        r#"{
        "text": "hello",
        "color": "red"
    }"#,
    )
    .unwrap();
    let component = Component::new(&j).unwrap();
    println!("println: {}", component.to_ansi(None));
}
