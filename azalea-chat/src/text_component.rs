use crate::{base_component::BaseComponent, style::ChatFormatting, Component};
use serde::{ser::SerializeMap, Serialize, Serializer, __private::ser::FlatMapSerializer};
use std::fmt::Display;

/// A component that contains text that's the same in all locales.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TextComponent {
    pub base: BaseComponent,
    pub text: String,
}

impl Serialize for TextComponent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("text", &self.text)?;
        Serialize::serialize(&self.base, FlatMapSerializer(&mut state))?;
        if !self.base.siblings.is_empty() {
            state.serialize_entry("extra", &self.base.siblings)?;
        }
        state.end()
    }
}

const LEGACY_FORMATTING_CODE_SYMBOL: char = '§';

/// Convert a legacy color code string into a Component
/// Technically in Minecraft this is done when displaying the text, but AFAIK it's the same as just doing it in TextComponent
pub fn legacy_color_code_to_text_component(legacy_color_code: &str) -> TextComponent {
    let mut components: Vec<TextComponent> = Vec::with_capacity(1);
    // iterate over legacy_color_code, if it starts with LEGACY_COLOR_CODE_SYMBOL then read the next character and get the style from that
    // otherwise, add the character to the text

    // we don't use a normal for loop since we need to be able to skip after reading the formatter code symbol
    let mut i = 0;
    while i < legacy_color_code.chars().count() {
        if legacy_color_code.chars().nth(i).unwrap() == LEGACY_FORMATTING_CODE_SYMBOL {
            let formatting_code = legacy_color_code.chars().nth(i + 1);
            let formatting_code = match formatting_code {
                Some(formatting_code) => formatting_code,
                None => {
                    i += 1;
                    continue;
                }
            };
            if let Some(formatter) = ChatFormatting::from_code(formatting_code) {
                if components.is_empty() || !components.last().unwrap().text.is_empty() {
                    components.push(TextComponent::new("".to_string()));
                }

                let style = &mut components.last_mut().unwrap().base.style;
                // if the formatter is a reset, then we need to reset the style to the default
                style.apply_formatting(&formatter);
            }
            i += 1;
        } else {
            if components.is_empty() {
                components.push(TextComponent::new("".to_string()));
            }
            components
                .last_mut()
                .unwrap()
                .text
                .push(legacy_color_code.chars().nth(i).unwrap());
        };
        i += 1;
    }

    if components.is_empty() {
        return TextComponent::new("".to_string());
    }

    // create the final component by using the first one as the base, and then adding the rest as siblings
    let mut final_component = components.remove(0);
    for component in components {
        final_component.base.siblings.push(component.get());
    }

    final_component
}

impl TextComponent {
    pub fn new(text: String) -> Self {
        // if it contains a LEGACY_FORMATTING_CODE_SYMBOL, format it
        if text.contains(LEGACY_FORMATTING_CODE_SYMBOL) {
            legacy_color_code_to_text_component(&text)
        } else {
            Self {
                base: BaseComponent::new(),
                text,
            }
        }
    }

    fn get(self) -> Component {
        Component::Text(self)
    }
}

impl Display for TextComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // this contains the final string will all the ansi escape codes
        for component in Component::Text(self.clone()).into_iter() {
            let component_text = match &component {
                Component::Text(c) => c.text.to_string(),
                Component::Translatable(c) => c.read()?.to_string(),
            };

            f.write_str(&component_text)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::style::Ansi;

    use super::*;

    #[test]
    fn test_hypixel_motd() {
        let component =
            TextComponent::new("§aHypixel Network  §c[1.8-1.18]\n§b§lHAPPY HOLIDAYS".to_string())
                .get();
        assert_eq!(
            component.to_ansi(),
            format!(
                "{GREEN}Hypixel Network  {RED}[1.8-1.18]\n{BOLD}{AQUA}HAPPY HOLIDAYS{RESET}",
                GREEN = Ansi::rgb(ChatFormatting::Green.color().unwrap()),
                RED = Ansi::rgb(ChatFormatting::Red.color().unwrap()),
                AQUA = Ansi::rgb(ChatFormatting::Aqua.color().unwrap()),
                BOLD = Ansi::BOLD,
                RESET = Ansi::RESET
            )
        );
    }

    #[test]
    fn test_legacy_color_code_to_component() {
        let component = TextComponent::new("§lHello §r§1w§2o§3r§4l§5d".to_string()).get();
        assert_eq!(
            component.to_ansi(),
            format!(
                "{BOLD}Hello {RESET}{DARK_BLUE}w{DARK_GREEN}o{DARK_AQUA}r{DARK_RED}l{DARK_PURPLE}d{RESET}",
                BOLD = Ansi::BOLD,
                RESET = Ansi::RESET,
                DARK_BLUE = Ansi::rgb(ChatFormatting::DarkBlue.color().unwrap()),
                DARK_GREEN = Ansi::rgb(ChatFormatting::DarkGreen.color().unwrap()),
                DARK_AQUA = Ansi::rgb(ChatFormatting::DarkAqua.color().unwrap()),
                DARK_RED = Ansi::rgb(ChatFormatting::DarkRed.color().unwrap()),
                DARK_PURPLE = Ansi::rgb(ChatFormatting::DarkPurple.color().unwrap())
            )
        );
    }
}
