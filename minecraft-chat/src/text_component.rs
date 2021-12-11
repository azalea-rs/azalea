use crate::{base_component::BaseComponent, component::Component, style::ChatFormatting};

#[derive(Clone, Debug)]
pub struct TextComponent {
    pub base: BaseComponent,
    pub text: String,
}

const LEGACY_FORMATTING_CODE_SYMBOL: char = '§';

/// Convert a legacy color code string into a Component
/// Technically in Minecraft this is done when displaying the text, but AFAIK it's the same as just doing it in TextComponent
pub fn legacy_color_code_to_component(legacy_color_code: &str) -> Component {
    let mut components: Vec<TextComponent> = Vec::with_capacity(1);
    // iterate over legacy_color_code, if it starts with LEGACY_COLOR_CODE_SYMBOL then read the next character and get the style from that
    // otherwise, add the character to the text

    // we don't use a normal for loop since we need to be able to skip after reading the formatter code symbol
    let mut i = 0;
    while i < legacy_color_code.chars().count() {
        if legacy_color_code.chars().nth(i).unwrap() == LEGACY_FORMATTING_CODE_SYMBOL {
            let formatting_code = legacy_color_code.chars().nth(i + 1).unwrap();
            if let Ok(formatter) = ChatFormatting::from_code(formatting_code) {
                if components.is_empty() || components.last().unwrap().text.is_empty() {
                    components.push(TextComponent::new("".to_string()));
                }
                println!(
                    "applying formatter {:?} {:?}",
                    components.last_mut().unwrap().base.style,
                    formatter
                );
                components
                    .last_mut()
                    .unwrap()
                    .base
                    .style
                    .apply_formatting(formatter);
                println!(
                    "applied formatter {:?}",
                    components.last_mut().unwrap().base.style
                );
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

    final_component.get()
}

impl<'a> TextComponent {
    pub fn new(text: String) -> Self {
        Self {
            base: BaseComponent::new(),
            text,
        }
    }

    pub fn to_string(&self) -> String {
        self.text.clone()
    }

    fn get(self) -> Component {
        Component::Text(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legacy_color_code_to_component() {
        let component = legacy_color_code_to_component("§lHello §r§1w§2o§3r§4l§5d");
        assert_eq!(
            component.to_ansi(),
            "\u{1b}[38;2;170;0;170mHello world\u{1b}[m"
        );
    }
}
