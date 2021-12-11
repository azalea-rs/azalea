use crate::{component::Component, style::Style};

#[derive(Clone, Debug)]
pub struct BaseComponent {
    // implements mutablecomponent
    pub siblings: Vec<Component>,
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
