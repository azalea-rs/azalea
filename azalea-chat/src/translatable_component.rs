use std::fmt::{self, Formatter};

use crate::{base_component::BaseComponent, component::Component};

#[derive(Clone, Debug)]
pub enum StringOrComponent {
    String(String),
    Component(Component),
}

#[derive(Clone, Debug)]
pub struct TranslatableComponent {
    pub base: BaseComponent,
    pub key: String,
    pub args: Vec<StringOrComponent>,
}

impl TranslatableComponent {
    pub fn new(key: String, args: Vec<StringOrComponent>) -> Self {
        Self {
            base: BaseComponent::new(),
            key,
            args,
        }
    }

    pub fn read(&self) -> Result<String, fmt::Error> {
        let template = azalea_language::get(&self.key).unwrap_or_else(|| &self.key);
        // decode the % things

        let mut result = String::new();
        let mut i = 0;
        let mut matched = 0;

        // this code is ugly but it works

        while i < template.len() {
            if template.chars().nth(i).unwrap() == '%' {
                let char_after = match template.chars().nth(i + 1) {
                    Some(c) => c,
                    None => {
                        result.push(template.chars().nth(i).unwrap());
                        break;
                    }
                };
                i += 1;
                match char_after {
                    '%' => {
                        result.push('%');
                    }
                    's' => {
                        result.push_str(
                            &self
                                .args
                                .get(matched)
                                .unwrap_or(&StringOrComponent::String("".to_string()))
                                .to_string(),
                        );
                        matched += 1;
                    }
                    _ => {
                        // check if the char is a number
                        if let Some(d) = char_after.to_digit(10) {
                            // make sure the next two chars are $s
                            if let Some('$') = template.chars().nth(i + 1) {
                                if let Some('s') = template.chars().nth(i + 2) {
                                    i += 2;
                                    result.push_str(
                                        &self
                                            .args
                                            .get((d - 1) as usize)
                                            .unwrap_or(&StringOrComponent::String("".to_string()))
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
                            result.push('%');
                        }
                    }
                }
            } else {
                result.push(template.chars().nth(i).unwrap());
            }

            i += 1
        }

        Ok(result.to_string())
    }
}

impl fmt::Display for TranslatableComponent {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.read()?)
    }
}

impl fmt::Display for StringOrComponent {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            StringOrComponent::String(s) => write!(f, "{}", s),
            StringOrComponent::Component(c) => write!(f, "{}", c.to_ansi(None)),
        }
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_none() {
        let c = TranslatableComponent::new("translation.test.none".to_string(), vec![]);
        assert_eq!(c.read(), Ok("Hello, world!".to_string()));
    }
    #[test]
    fn test_complex() {
        let c = TranslatableComponent::new(
            "translation.test.complex".to_string(),
            vec![
                StringOrComponent::String("a".to_string()),
                StringOrComponent::String("b".to_string()),
                StringOrComponent::String("c".to_string()),
                StringOrComponent::String("d".to_string()),
            ],
        );
        // so true mojang
        assert_eq!(
            c.read(),
            Ok("Prefix, ab again b and a lastly c and also a again!".to_string())
        );
    }
    #[test]
    fn test_escape() {
        let c = TranslatableComponent::new(
            "translation.test.escape".to_string(),
            vec![
                StringOrComponent::String("a".to_string()),
                StringOrComponent::String("b".to_string()),
                StringOrComponent::String("c".to_string()),
                StringOrComponent::String("d".to_string()),
            ],
        );
        assert_eq!(c.read(), Ok("%s %a %%s %%b".to_string()));
    }
    #[test]
    fn test_invalid() {
        let c = TranslatableComponent::new(
            "translation.test.invalid".to_string(),
            vec![
                StringOrComponent::String("a".to_string()),
                StringOrComponent::String("b".to_string()),
                StringOrComponent::String("c".to_string()),
                StringOrComponent::String("d".to_string()),
            ],
        );
        assert_eq!(c.read(), Ok("hi %".to_string()));
    }
    #[test]
    fn test_invalid2() {
        let c = TranslatableComponent::new(
            "translation.test.invalid2".to_string(),
            vec![
                StringOrComponent::String("a".to_string()),
                StringOrComponent::String("b".to_string()),
                StringOrComponent::String("c".to_string()),
                StringOrComponent::String("d".to_string()),
            ],
        );
        assert_eq!(c.read(), Ok("hi %  s".to_string()));
    }
}
