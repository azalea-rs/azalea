#![doc = include_str!("../README.md")]

use std::{collections::HashMap, sync::LazyLock};

pub static STORAGE: LazyLock<HashMap<String, String>> =
    LazyLock::new(|| serde_json::from_str(include_str!("en_us.json")).unwrap());

pub fn get(key: &str) -> Option<&str> {
    STORAGE.get(key).map(|s| s.as_str())
}
