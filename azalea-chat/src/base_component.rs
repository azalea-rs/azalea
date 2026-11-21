use azalea_codec::Codec;

use crate::{FormattedText, style::Style};

#[derive(Clone, Debug, PartialEq, Codec)]
pub struct BaseComponent {
    /// Components in the "extra" field.
    #[codec(optional, rename = "extra")]
    pub siblings: Vec<FormattedText>,
    #[codec(flatten)]
    pub style: Box<Style>,
}

impl BaseComponent {
    pub fn new() -> Self {
        Self {
            siblings: Vec::new(),
            style: Default::default(),
        }
    }
    pub fn with_style(self, style: Style) -> Self {
        Self {
            style: Box::new(style),
            ..self
        }
    }
    /// Whether this base component has no siblings and the default style.
    pub fn is_empty(&self) -> bool {
        self == &BaseComponent::new()
    }
}

impl Default for BaseComponent {
    fn default() -> Self {
        Self::new()
    }
}
