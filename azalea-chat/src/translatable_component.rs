use std::fmt::{self, Display};

use azalea_codec::Codec;

use crate::{FormattedText, base_component::BaseComponent, text_component::TextComponent};

#[derive(Clone, Debug, PartialEq, Codec)]
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

/// A message whose content depends on the client's language.
#[derive(Clone, Debug, PartialEq, Codec)]
pub struct TranslatableComponent {
    #[codec(rename = "translate")]
    pub key: String,
    #[codec(optional)]
    pub fallback: Option<String>,
    #[codec(optional, rename = "with")]
    pub args: Vec<PrimitiveOrComponent>,
    #[codec(flatten)]
    pub base: BaseComponent,
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

    /// Parse the key and args, and convert this component into a
    /// `TextComponent`, including siblings.
    pub fn into_text_component(self) -> Result<TextComponent, fmt::Error> {
        let template = azalea_language::get(&self.key).unwrap_or_else(|| {
            if let Some(fallback) = &self.fallback {
                fallback.as_str()
            } else {
                &self.key
            }
        });
        // decode the % things

        let mut matched = 0;

        // every time we get a char we add it to built_text, and we push it to
        // `arguments` and clear it when we add a new argument component
        let mut built_text = String::new();
        let mut components = Vec::new();

        let mut chars = template.chars();
        while let Some(char) = chars.next() {
            if char != '%' {
                built_text.push(char);
                continue;
            }

            let mut chars_preview = chars.clone();
            let Some(char_after) = chars_preview.next() else {
                built_text.push('%');
                break;
            };
            match char_after {
                '%' => {
                    chars.next();

                    built_text.push('%');
                }
                's' => {
                    chars.next();

                    let arg_component = self
                        .args
                        .get(matched)
                        .cloned()
                        .unwrap_or_else(|| PrimitiveOrComponent::String("".to_owned()));

                    components.push(TextComponent::new(built_text.clone()));
                    built_text.clear();
                    components.push(TextComponent::from(arg_component));
                    matched += 1;
                }
                '0'..='9' if let Some(d) = char_after.to_digit(10) => {
                    chars.next();
                    // make sure the next two chars are $s
                    let Some('$') = chars.next() else {
                        return Err(fmt::Error);
                    };
                    let Some('s') = chars.next() else {
                        return Err(fmt::Error);
                    };
                    built_text.push_str(
                        &self
                            .args
                            .get((d - 1) as usize)
                            .unwrap_or(&PrimitiveOrComponent::String("".to_owned()))
                            .to_string(),
                    );
                }
                _ => {
                    built_text.push('%');
                }
            }
        }

        components.push(TextComponent::new(built_text));

        Ok(
            TextComponent::empty_with_text_component_siblings(components)
                .with_style(*self.base.style)
                .with_additional_siblings(self.base.siblings),
        )
    }

    #[doc(hidden)]
    #[deprecated = "renamed to `to_text_component`"]
    pub fn read(&self) -> Result<TextComponent, fmt::Error> {
        Self::into_text_component(self.clone())
    }
}

impl Display for TranslatableComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        FormattedText::Translatable(self.clone()).fmt(f)
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
        TranslatableComponent::new(s.to_owned(), vec![])
    }
}
impl From<TranslatableComponent> for FormattedText {
    fn from(c: TranslatableComponent) -> Self {
        FormattedText::Translatable(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_none() {
        let c = TranslatableComponent::new("translation.test.none".to_owned(), vec![]);
        assert_eq!(
            c.into_text_component().unwrap().to_string(),
            "Hello, world!".to_owned()
        );
    }
    #[test]
    fn test_complex() {
        let c = TranslatableComponent::new(
            "translation.test.complex".to_owned(),
            vec![
                PrimitiveOrComponent::String("a".to_owned()),
                PrimitiveOrComponent::String("b".to_owned()),
                PrimitiveOrComponent::String("c".to_owned()),
                PrimitiveOrComponent::String("d".to_owned()),
            ],
        );
        // so true mojang
        assert_eq!(
            c.into_text_component().unwrap().to_string(),
            "Prefix, ab again b and a lastly c and also a again!".to_owned()
        );
    }
    #[test]
    fn test_escape() {
        let c = TranslatableComponent::new(
            "translation.test.escape".to_owned(),
            vec![
                PrimitiveOrComponent::String("a".to_owned()),
                PrimitiveOrComponent::String("b".to_owned()),
                PrimitiveOrComponent::String("c".to_owned()),
                PrimitiveOrComponent::String("d".to_owned()),
            ],
        );
        assert_eq!(
            c.into_text_component().unwrap().to_string(),
            "%s %a %%s %%b".to_owned()
        );
    }
    #[test]
    fn test_invalid() {
        let c = TranslatableComponent::new(
            "translation.test.invalid".to_owned(),
            vec![
                PrimitiveOrComponent::String("a".to_owned()),
                PrimitiveOrComponent::String("b".to_owned()),
                PrimitiveOrComponent::String("c".to_owned()),
                PrimitiveOrComponent::String("d".to_owned()),
            ],
        );
        assert_eq!(c.to_string(), "hi %".to_owned());
    }
    #[test]
    fn test_invalid2() {
        let c = TranslatableComponent::new(
            "translation.test.invalid2".to_owned(),
            vec![
                PrimitiveOrComponent::String("a".to_owned()),
                PrimitiveOrComponent::String("b".to_owned()),
                PrimitiveOrComponent::String("c".to_owned()),
                PrimitiveOrComponent::String("d".to_owned()),
            ],
        );
        assert_eq!(c.to_string(), "hi %  s".to_owned());
    }

    #[test]
    fn test_undefined() {
        let c = TranslatableComponent::new(
            "translation.test.undefined".to_owned(),
            vec![PrimitiveOrComponent::String("a".to_owned())],
        );
        assert_eq!(c.to_string(), "translation.test.undefined".to_owned());
    }

    #[test]
    fn test_undefined_with_fallback() {
        let c = TranslatableComponent::with_fallback(
            "translation.test.undefined".to_owned(),
            Some("translation fallback: %s".to_owned()),
            vec![PrimitiveOrComponent::String("a".to_owned())],
        );
        assert_eq!(c.to_string(), "translation fallback: a".to_owned());
    }
}
