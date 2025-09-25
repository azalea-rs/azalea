use serde::{Serialize, ser::SerializeMap};

use crate::{FormattedText, style::Style};

#[derive(Clone, Debug, PartialEq)]
pub struct BaseComponent {
    /// Components in the "extra" field.
    pub siblings: Vec<FormattedText>,
    pub style: Box<Style>,
}

impl BaseComponent {
    pub fn serialize_map<S>(&self, state: &mut S::SerializeMap) -> Result<(), S::Error>
    where
        S: serde::Serializer,
    {
        if !self.siblings.is_empty() {
            state.serialize_entry("extra", &self.siblings)?;
        }
        self.style.serialize_map::<S>(state)?;
        Ok(())
    }
}
impl Serialize for BaseComponent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_map(None)?;
        self.serialize_map::<S>(&mut state)?;
        state.end()
    }
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
