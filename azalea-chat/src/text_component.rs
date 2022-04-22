use std::fmt;

use crate::{base_component::BaseComponent, component::Component, style::ChatFormatting};

#[derive(Clone, Debug)]
pub struct TextComponent {
    pub base: BaseComponent,
    pub text: String,
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
            let formatting_code = legacy_color_code.chars().nth(i + 1).unwrap();
            if let Ok(formatter) = ChatFormatting::from_code(formatting_code) {
                if components.is_empty() || !components.last().unwrap().text.is_empty() {
                    components.push(TextComponent::new("".to_string()));
                }

                let style = &mut components.last_mut().unwrap().base.style;
                // if the formatter is a reset, then we need to reset the style to the default
                style.apply_formatting(formatter);
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

    // create the final component by using the first one as the base, and then adding the rest as siblings
    let mut final_component = components.remove(0);
    for component in components {
        final_component.base.siblings.push(component.get());
    }

    final_component
}

impl<'a> TextComponent {
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

impl fmt::Display for TextComponent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text.clone())
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
            component.to_ansi(None),
            format!(
                "{GREEN}Hypixel Network  {RED}[1.8-1.18]\n{BOLD}{AQUA}HAPPY HOLIDAYS{RESET}",
                GREEN = Ansi::rgb(ChatFormatting::GREEN.color.unwrap()),
                RED = Ansi::rgb(ChatFormatting::RED.color.unwrap()),
                AQUA = Ansi::rgb(ChatFormatting::AQUA.color.unwrap()),
                BOLD = Ansi::BOLD,
                RESET = Ansi::RESET
            )
        );
    }

    #[test]
    fn test_legacy_color_code_to_component() {
        let component = TextComponent::new("§lHello §r§1w§2o§3r§4l§5d".to_string()).get();
        assert_eq!(
            component.to_ansi(None),
            format!(
                "{BOLD}Hello {RESET}{DARK_BLUE}w{DARK_GREEN}o{DARK_AQUA}r{DARK_RED}l{DARK_PURPLE}d{RESET}",
                BOLD = Ansi::BOLD,
                RESET = Ansi::RESET,
                DARK_BLUE = Ansi::rgb(ChatFormatting::DARK_BLUE.color.unwrap()),
                DARK_GREEN = Ansi::rgb(ChatFormatting::DARK_GREEN.color.unwrap()),
                DARK_AQUA = Ansi::rgb(ChatFormatting::DARK_AQUA.color.unwrap()),
                DARK_RED = Ansi::rgb(ChatFormatting::DARK_RED.color.unwrap()),
                DARK_PURPLE = Ansi::rgb(ChatFormatting::DARK_PURPLE.color.unwrap())
            )
        );
    }
}
