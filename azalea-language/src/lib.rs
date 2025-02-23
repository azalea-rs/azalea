#![doc = include_str!("../README.md")]

use std::{collections::HashMap, sync::LazyLock};

use compact_str::CompactString;

pub static STORAGE: LazyLock<HashMap<CompactString, CompactString>> =
    LazyLock::new(|| serde_json::from_str(include_str!("en_us.json")).unwrap());

pub fn get(key: &str) -> Option<&str> {
    STORAGE.get(key).map(|s| s.as_str())
}
