use serde::Serialize;

use crate::{style::Style, FormattedText};

#[derive(Clone, Debug, PartialEq, Serialize, Eq, Hash)]
pub struct BaseComponent {
    // implements mutablecomponent
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub siblings: Vec<FormattedText>,
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

#[cfg(feature = "simdnbt")]
impl simdnbt::Serialize for BaseComponent {
    fn to_compound(self) -> simdnbt::owned::NbtCompound {
        let mut compound = simdnbt::owned::NbtCompound::new();
        if !self.siblings.is_empty() {
            compound.insert(
                "extra",
                simdnbt::owned::NbtList::from(
                    self.siblings
                        .into_iter()
                        .map(|component| component.to_compound())
                        .collect::<Vec<_>>(),
                ),
            );
        }
        compound.extend(self.style.to_compound());
        compound
    }
}

impl Default for BaseComponent {
    fn default() -> Self {
        Self::new()
    }
}
