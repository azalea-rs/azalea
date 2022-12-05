use crate::{style::Style, Component};
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BaseComponent {
    // implements mutablecomponent
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub siblings: Vec<Component>,
    #[serde(flatten)]
    pub style: Style,
}

impl BaseComponent {
    pub fn new() -> Self {
        Self {
            siblings: Vec::new(),
            style: Style::default(),
        }
    }
}

impl Default for BaseComponent {
    fn default() -> Self {
        Self::new()
    }
}
