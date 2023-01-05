//! Translate Minecraft strings from their id.

use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static STORAGE: Lazy<HashMap<String, String>> =
    Lazy::new(|| serde_json::from_str(include_str!("en_us.json")).unwrap());

pub fn get(key: &str) -> Option<&str> {
    STORAGE.get(key).map(|s| s.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        assert_eq!(get("translation.test.none"), Some("Hello, world!"));
    }
}
