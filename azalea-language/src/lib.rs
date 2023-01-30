#![doc = include_str!("../README.md")]

use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static STORAGE: Lazy<HashMap<String, String>> =
    Lazy::new(|| serde_json::from_str(include_str!("en_us.json")).unwrap());

pub fn get(key: &str) -> Option<&str> {
    STORAGE.get(key).map(|s| s.as_str())
}
