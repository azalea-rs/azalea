# Azalea Chat

Things for working with Minecraft formatted text components.

# Examples

```
// convert a Minecraft formatted text JSON into colored text that can be printed to the terminal.

use azalea_chat::FormattedText;
use serde_json::Value;
use serde::Deserialize;

let j: Value = serde_json::from_str(
    r#"{"text": "hello","color": "red","bold": true}"#
)
.unwrap();
let text = FormattedText::deserialize(&j).unwrap();
assert_eq!(
    text.to_ansi(),
    "\u{1b}[1m\u{1b}[38;2;255;85;85mhello\u{1b}[m"
);
```
