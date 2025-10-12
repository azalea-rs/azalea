use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};
#[cfg(feature = "simdnbt")]
use simdnbt::{
    ToNbtTag,
    owned::{NbtList, NbtTag},
};

use crate::{FormattedText, base_component::BaseComponent, text_component::TextComponent};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrimitiveOrComponent {
    Boolean(bool),
    Short(i16),
    Integer(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    FormattedText(FormattedText),
}

#[cfg(feature = "simdnbt")]
impl simdnbt::ToNbtTag for PrimitiveOrComponent {
    fn to_nbt_tag(self) -> simdnbt::owned::NbtTag {
        match self {
            PrimitiveOrComponent::Boolean(value) => value.to_nbt_tag(),
            PrimitiveOrComponent::Short(value) => value.to_nbt_tag(),
            PrimitiveOrComponent::Integer(value) => value.to_nbt_tag(),
            PrimitiveOrComponent::Long(value) => value.to_nbt_tag(),
            PrimitiveOrComponent::Float(value) => value.to_nbt_tag(),
            PrimitiveOrComponent::Double(value) => value.to_nbt_tag(),
            PrimitiveOrComponent::String(value) => value.to_nbt_tag(),
            PrimitiveOrComponent::FormattedText(value) => value.to_nbt_tag(),
        }
    }
}

/// A message whose content depends on the client's language.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TranslatableComponent {
    #[serde(flatten)]
    pub base: BaseComponent,
    #[serde(rename = "translate")]
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback: Option<String>,
    #[serde(rename = "with")]
    pub args: Vec<PrimitiveOrComponent>,
}

#[cfg(feature = "simdnbt")]
fn serialize_args_as_nbt(args: Vec<PrimitiveOrComponent>) -> NbtList {
    let tags = args
        .into_iter()
        .map(|arg| arg.to_nbt_tag())
        .collect::<Vec<NbtTag>>();
    NbtList::from(tags)
}

#[cfg(feature = "simdnbt")]
impl simdnbt::Serialize for TranslatableComponent {
    fn to_compound(self) -> simdnbt::owned::NbtCompound {
        let mut compound = simdnbt::owned::NbtCompound::new();
        compound.insert("translate", self.key);
        compound.extend(self.base.style.to_compound());

        compound.insert("with", serialize_args_as_nbt(self.args));
        compound
    }
}

impl TranslatableComponent {
    pub fn new(key: String, args: Vec<PrimitiveOrComponent>) -> Self {
        Self {
            base: BaseComponent::new(),
            key,
            fallback: None,
            args,
        }
    }

    pub fn with_fallback(
        key: String,
        fallback: Option<String>,
        args: Vec<PrimitiveOrComponent>,
    ) -> Self {
        Self {
            base: BaseComponent::new(),
            key,
            fallback,
            args,
        }
    }

    /// Convert the key and args to a FormattedText.
    pub fn read(&self) -> Result<TextComponent, fmt::Error> {
        let template = azalea_language::get(&self.key).unwrap_or_else(|| {
            if let Some(fallback) = &self.fallback {
                fallback.as_str()
            } else {
                &self.key
            }
        });
        // decode the % things

        let mut i = 0;
        let mut matched = 0;

        // every time we get a char we add it to built_text, and we push it to
        // `arguments` and clear it when we add a new argument component
        let mut built_text = String::new();
        let mut components = Vec::new();

        while i < template.chars().count() {
            if template.chars().nth(i).unwrap() == '%' {
                let Some(char_after) = template.chars().nth(i + 1) else {
                    built_text.push('%');
                    break;
                };
                i += 1;
                match char_after {
                    '%' => {
                        built_text.push('%');
                    }
                    's' => {
                        let arg_component = self
                            .args
                            .get(matched)
                            .cloned()
                            .unwrap_or_else(|| PrimitiveOrComponent::String("".to_string()));

                        components.push(TextComponent::new(built_text.clone()));
                        built_text.clear();
                        components.push(TextComponent::from(arg_component));
                        matched += 1;
                    }
                    _ => {
                        // check if the char is a number
                        if let Some(d) = char_after.to_digit(10) {
                            // make sure the next two chars are $s
                            if let Some('$') = template.chars().nth(i + 1) {
                                if let Some('s') = template.chars().nth(i + 2) {
                                    i += 2;
                                    built_text.push_str(
                                        &self
                                            .args
                                            .get((d - 1) as usize)
                                            .unwrap_or(&PrimitiveOrComponent::String(
                                                "".to_string(),
                                            ))
                                            .to_string(),
                                    );
                                } else {
                                    return Err(fmt::Error);
                                }
                            } else {
                                return Err(fmt::Error);
                            }
                        } else {
                            i -= 1;
                            built_text.push('%');
                        }
                    }
                }
            } else {
                built_text.push(template.chars().nth(i).unwrap());
            }

            i += 1;
        }

        if components.is_empty() {
            return Ok(TextComponent::new(built_text));
        }

        components.push(TextComponent::new(built_text));

        Ok(TextComponent {
            base: BaseComponent {
                siblings: components.into_iter().map(FormattedText::Text).collect(),
                style: Default::default(),
            },
            text: "".to_string(),
        })
    }
}

impl Display for TranslatableComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // this contains the final string will all the ansi escape codes
        for component in FormattedText::Translatable(self.clone()).into_iter() {
            let component_text = match &component {
                FormattedText::Text(c) => c.text.to_string(),
                FormattedText::Translatable(c) => match c.read() {
                    Ok(c) => c.to_string(),
                    Err(_) => c.key.to_string(),
                },
            };

            f.write_str(&component_text)?;
        }

        Ok(())
    }
}

impl Display for PrimitiveOrComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            PrimitiveOrComponent::Boolean(value) => write!(f, "{value}"),
            PrimitiveOrComponent::Short(value) => write!(f, "{value}"),
            PrimitiveOrComponent::Integer(value) => write!(f, "{value}"),
            PrimitiveOrComponent::Long(value) => write!(f, "{value}"),
            PrimitiveOrComponent::Float(value) => write!(f, "{value}"),
            PrimitiveOrComponent::Double(value) => write!(f, "{value}"),
            PrimitiveOrComponent::String(value) => write!(f, "{value}"),
            PrimitiveOrComponent::FormattedText(value) => write!(f, "{value}"),
        }
    }
}

impl From<PrimitiveOrComponent> for TextComponent {
    fn from(soc: PrimitiveOrComponent) -> Self {
        match soc {
            PrimitiveOrComponent::String(value) => TextComponent::new(value),
            PrimitiveOrComponent::Boolean(value) => TextComponent::new(value.to_string()),
            PrimitiveOrComponent::Short(value) => TextComponent::new(value.to_string()),
            PrimitiveOrComponent::Integer(value) => TextComponent::new(value.to_string()),
            PrimitiveOrComponent::Long(value) => TextComponent::new(value.to_string()),
            PrimitiveOrComponent::Float(value) => TextComponent::new(value.to_string()),
            PrimitiveOrComponent::Double(value) => TextComponent::new(value.to_string()),
            PrimitiveOrComponent::FormattedText(value) => TextComponent::new(value.to_string()),
        }
    }
}
impl From<&str> for TranslatableComponent {
    fn from(s: &str) -> Self {
        TranslatableComponent::new(s.to_string(), vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_none() {
        let c = TranslatableComponent::new("translation.test.none".to_string(), vec![]);
        assert_eq!(c.read().unwrap().to_string(), "Hello, world!".to_string());
    }
    #[test]
    fn test_complex() {
        let c = TranslatableComponent::new(
            "translation.test.complex".to_string(),
            vec![
                PrimitiveOrComponent::String("a".to_string()),
                PrimitiveOrComponent::String("b".to_string()),
                PrimitiveOrComponent::String("c".to_string()),
                PrimitiveOrComponent::String("d".to_string()),
            ],
        );
        // so true mojang
        assert_eq!(
            c.read().unwrap().to_string(),
            "Prefix, ab again b and a lastly c and also a again!".to_string()
        );
    }
    #[test]
    fn test_escape() {
        let c = TranslatableComponent::new(
            "translation.test.escape".to_string(),
            vec![
                PrimitiveOrComponent::String("a".to_string()),
                PrimitiveOrComponent::String("b".to_string()),
                PrimitiveOrComponent::String("c".to_string()),
                PrimitiveOrComponent::String("d".to_string()),
            ],
        );
        assert_eq!(c.read().unwrap().to_string(), "%s %a %%s %%b".to_string());
    }
    #[test]
    fn test_invalid() {
        let c = TranslatableComponent::new(
            "translation.test.invalid".to_string(),
            vec![
                PrimitiveOrComponent::String("a".to_string()),
                PrimitiveOrComponent::String("b".to_string()),
                PrimitiveOrComponent::String("c".to_string()),
                PrimitiveOrComponent::String("d".to_string()),
            ],
        );
        assert_eq!(c.read().unwrap().to_string(), "hi %".to_string());
    }
    #[test]
    fn test_invalid2() {
        let c = TranslatableComponent::new(
            "translation.test.invalid2".to_string(),
            vec![
                PrimitiveOrComponent::String("a".to_string()),
                PrimitiveOrComponent::String("b".to_string()),
                PrimitiveOrComponent::String("c".to_string()),
                PrimitiveOrComponent::String("d".to_string()),
            ],
        );
        assert_eq!(c.read().unwrap().to_string(), "hi %  s".to_string());
    }

    #[test]
    fn test_undefined() {
        let c = TranslatableComponent::new(
            "translation.test.undefined".to_string(),
            vec![PrimitiveOrComponent::String("a".to_string())],
        );
        assert_eq!(
            c.read().unwrap().to_string(),
            "translation.test.undefined".to_string()
        );
    }

    #[test]
    fn test_undefined_with_fallback() {
        let c = TranslatableComponent::with_fallback(
            "translation.test.undefined".to_string(),
            Some("translation fallback: %s".to_string()),
            vec![PrimitiveOrComponent::String("a".to_string())],
        );
        assert_eq!(
            c.read().unwrap().to_string(),
            "translation fallback: a".to_string()
        );
    }
}
