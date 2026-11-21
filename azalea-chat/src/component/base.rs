use std::fmt::{self, Debug};

use azalea_codec::Codec;

use crate::{FormattedText, style::Style};

#[derive(Clone, PartialEq, Codec)]
pub struct BaseComponent {
    /// Components in the "extra" field.
    #[codec(optional, rename = "extra")]
    pub siblings: Vec<FormattedText>,
    #[codec(optional, flatten)]
    pub style: Box<Style>,

    #[codec(flatten)]
    pub _unused: azalea_codec::types::Drain,
}

impl BaseComponent {
    pub fn new() -> Self {
        Self {
            siblings: Vec::new(),
            style: Default::default(),
            _unused: Default::default(),
        }
    }
    pub fn with_style(self, style: Style) -> Self {
        Self {
            style: Box::new(style),
            ..self
        }
    }
    /// Whether this base component has no siblings and the default style.
    pub fn is_default(&self) -> bool {
        self == &BaseComponent::new()
    }
}

impl Default for BaseComponent {
    fn default() -> Self {
        Self::new()
    }
}
impl Debug for BaseComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("BaseComponent");
        if !self.siblings.is_empty() {
            s.field("siblings", &self.siblings);
        }
        if !self.style.is_empty() {
            s.field("style", &self.style);
        }
        s.finish()
    }
}
