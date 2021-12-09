use std::collections::HashMap;

use serde_json::Value;

#[derive(Clone, PartialEq, Debug)]
pub struct TextColor {
    value: u32,
    name: Option<String>,
}

impl TextColor {
    pub fn parse(value: String) -> Result<TextColor, String> {
        // if (string.startsWith("#")) {
        //     try {
        //         int n = Integer.parseInt(string.substring(1), 16);
        //         return TextColor.fromRgb(n);
        //     }
        //     catch (NumberFormatException numberFormatException) {
        //         return null;
        //     }
        // }
        // return NAMED_COLORS.get(string);
        if value.starts_with("#") {
            let n = value.chars().skip(1).collect::<String>();
            let n = u32::from_str_radix(&n, 16).unwrap();
            return Ok(TextColor::from_rgb(n));
        }
        // private static final Map<ChatFormatting, TextColor> LEGACY_FORMAT_TO_COLOR = (Map)Stream.of(ChatFormatting.values()).filter(ChatFormatting::isColor).collect(ImmutableMap.toImmutableMap(Function.identity(), chatFormatting -> new TextColor(chatFormatting.getColor(), chatFormatting.getName())));
        // private static final Map<String, TextColor> NAMED_COLORS = (Map)LEGACY_FORMAT_TO_COLOR.values().stream().collect(ImmutableMap.toImmutableMap(textColor -> textColor.name, Function.identity()));
        let mut LEGACY_FORMAT_TO_COLOR = HashMap::new();
        let mut NAMED_COLORS = HashMap::new();
        for formatter in &ChatFormatting::FORMATTERS {
            if !formatter.is_format && *formatter != ChatFormatting::RESET {
                LEGACY_FORMAT_TO_COLOR.insert(
                    formatter,
                    TextColor {
                        value: formatter.color.unwrap(),
                        name: Some(formatter.name.to_string()),
                    },
                );
            }
        }
        for color in LEGACY_FORMAT_TO_COLOR.values() {
            NAMED_COLORS.insert(color.name.as_ref().unwrap(), color.clone());
        }
        let color = NAMED_COLORS.get(&value.to_ascii_uppercase());
        if color.is_some() {
            return Ok(color.unwrap().clone());
        }
        Err(format!("Invalid color {}", value))
    }

    fn from_rgb(value: u32) -> TextColor {
        TextColor { value, name: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_color_named_colors() {
        assert_eq!(
            TextColor::parse("red".to_string()).unwrap().value,
            16733525u32
        );
    }
}

const PREFIX_CODE: char = '\u{00a7}';

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct ChatFormatting<'a> {
    name: &'a str,
    code: char,
    is_format: bool,
    id: i32,
    color: Option<u32>,
}

impl<'a> ChatFormatting<'a> {
    const BLACK: ChatFormatting<'a> = ChatFormatting::new("BLACK", '0', false, 0, Some(0));
    const DARK_BLUE: ChatFormatting<'a> =
        ChatFormatting::new("DARK_BLUE", '1', false, 1, Some(170));
    const DARK_GREEN: ChatFormatting<'a> =
        ChatFormatting::new("DARK_GREEN", '2', false, 2, Some(43520));
    const DARK_AQUA: ChatFormatting<'a> =
        ChatFormatting::new("DARK_AQUA", '3', false, 3, Some(43690));
    const DARK_RED: ChatFormatting<'a> =
        ChatFormatting::new("DARK_RED", '4', false, 4, Some(1114112));
    const DARK_PURPLE: ChatFormatting<'a> =
        ChatFormatting::new("DARK_PURPLE", '5', false, 5, Some(11141290));
    const GOLD: ChatFormatting<'a> = ChatFormatting::new("GOLD", '6', false, 6, Some(16755200));
    const GRAY: ChatFormatting<'a> = ChatFormatting::new("GRAY", '7', false, 7, Some(11184810));
    const DARK_GRAY: ChatFormatting<'a> =
        ChatFormatting::new("DARK_GRAY", '8', false, 8, Some(5592405));
    const BLUE: ChatFormatting<'a> = ChatFormatting::new("BLUE", '9', false, 9, Some(5592575));
    const GREEN: ChatFormatting<'a> = ChatFormatting::new("GREEN", 'a', false, 10, Some(5635925));
    const AQUA: ChatFormatting<'a> = ChatFormatting::new("AQUA", 'b', false, 11, Some(5636095));
    const RED: ChatFormatting<'a> = ChatFormatting::new("RED", 'c', false, 12, Some(16733525));
    const LIGHT_PURPLE: ChatFormatting<'a> =
        ChatFormatting::new("LIGHT_PURPLE", 'd', false, 13, Some(16733695));
    const YELLOW: ChatFormatting<'a> =
        ChatFormatting::new("YELLOW", 'e', false, 14, Some(16777045));
    const WHITE: ChatFormatting<'a> = ChatFormatting::new("WHITE", 'f', false, 15, Some(16777215));
    const OBFUSCATED: ChatFormatting<'a> = ChatFormatting::new("OBFUSCATED", 'k', true, -1, None);
    const STRIKETHROUGH: ChatFormatting<'a> =
        ChatFormatting::new("STRIKETHROUGH", 'm', true, -1, None);
    const BOLD: ChatFormatting<'a> = ChatFormatting::new("BOLD", 'l', true, -1, None);
    const UNDERLINE: ChatFormatting<'a> = ChatFormatting::new("UNDERLINE", 'n', true, -1, None);
    const ITALIC: ChatFormatting<'a> = ChatFormatting::new("ITALIC", 'o', true, -1, None);
    const RESET: ChatFormatting<'a> = ChatFormatting::new("RESET", 'r', true, -1, None);

    pub const FORMATTERS: [ChatFormatting<'a>; 16] = [
        ChatFormatting::BLACK,
        ChatFormatting::DARK_BLUE,
        ChatFormatting::DARK_GREEN,
        ChatFormatting::DARK_AQUA,
        ChatFormatting::DARK_RED,
        ChatFormatting::DARK_PURPLE,
        ChatFormatting::GOLD,
        ChatFormatting::GRAY,
        ChatFormatting::DARK_GRAY,
        ChatFormatting::BLUE,
        ChatFormatting::GREEN,
        ChatFormatting::AQUA,
        ChatFormatting::RED,
        ChatFormatting::LIGHT_PURPLE,
        ChatFormatting::YELLOW,
        ChatFormatting::WHITE,
    ];

    const fn new(
        name: &str,
        code: char,
        is_format: bool,
        id: i32,
        color: Option<u32>,
    ) -> ChatFormatting {
        ChatFormatting {
            name: name,
            code,
            is_format,
            id,
            color,
        }
    }
}

impl TextColor {
    fn new(value: u32, name: Option<String>) -> Self {
        Self { value, name }
    }

    pub fn format(&self) -> String {
        format!("#{:06X}", self.value)
    }

    pub fn to_string(&self) -> String {
        if let Some(name) = &self.name {
            name.clone()
        } else {
            self.format()
        }
    }
}

#[derive(Clone, Debug)]
pub struct Style {
    // @Nullable
    // final TextColor color;
    // @Nullable
    // final Boolean bold;
    // @Nullable
    // final Boolean italic;
    // @Nullable
    // final Boolean underlined;
    // @Nullable
    // final Boolean strikethrough;
    // @Nullable
    // final Boolean obfuscated;
    // @Nullable
    // final ClickEvent clickEvent;
    // @Nullable
    // final HoverEvent hoverEvent;
    // @Nullable
    // final String insertion;
    // @Nullable
    // final ResourceLocation font;

    // these are options instead of just bools because None is different than false in this case
    pub color: Option<TextColor>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underlined: Option<bool>,
    pub strikethrough: Option<bool>,
    pub obfuscated: Option<bool>,
}

impl Style {
    pub fn new() -> Style {
        Style {
            color: None,
            bold: None,
            italic: None,
            underlined: None,
            strikethrough: None,
            obfuscated: None,
        }
    }

    pub fn deserialize(json: &Value) -> Style {
        // if (jsonElement.isJsonObject()) {
        //     JsonObject jsonObject = jsonElement.getAsJsonObject();
        //     if (jsonObject == null) {
        //         return null;
        //     }
        //     Boolean bl = Serializer.getOptionalFlag(jsonObject, "bold");
        //     Boolean bl2 = Serializer.getOptionalFlag(jsonObject, "italic");
        //     Boolean bl3 = Serializer.getOptionalFlag(jsonObject, "underlined");
        //     Boolean bl4 = Serializer.getOptionalFlag(jsonObject, "strikethrough");
        //     Boolean bl5 = Serializer.getOptionalFlag(jsonObject, "obfuscated");
        //     TextColor textColor = Serializer.getTextColor(jsonObject);
        //     String string = Serializer.getInsertion(jsonObject);
        //     ClickEvent clickEvent = Serializer.getClickEvent(jsonObject);
        //     HoverEvent hoverEvent = Serializer.getHoverEvent(jsonObject);
        //     ResourceLocation resourceLocation = Serializer.getFont(jsonObject);
        //     return new Style(textColor, bl, bl2, bl3, bl4, bl5, clickEvent, hoverEvent, string, resourceLocation);
        // }
        // return null;
        return if json.is_object() {
            let json_object = json.as_object().unwrap();
            let bold = json_object.get("bold").and_then(|v| v.as_bool());
            let italic = json_object.get("italic").and_then(|v| v.as_bool());
            let underlined = json_object.get("underlined").and_then(|v| v.as_bool());
            let strikethrough = json_object.get("strikethrough").and_then(|v| v.as_bool());
            let obfuscated = json_object.get("obfuscated").and_then(|v| v.as_bool());
            let color: Option<TextColor> = json_object
                .get("color")
                .and_then(|v| v.as_str())
                .and_then(|v| TextColor::parse(v.to_string()).ok());
            Style {
                color,
                bold,
                italic,
                underlined,
                strikethrough,
                obfuscated,
            }
        } else {
            Style::new()
        };
    }

    /// Check if a style has no attributes set
    pub fn is_empty(&self) -> bool {
        self.color.is_none()
            && self.bold.is_none()
            && self.italic.is_none()
            && self.underlined.is_none()
            && self.strikethrough.is_none()
            && self.obfuscated.is_none()
    }

    /// find the necessary ansi code to get from this style to another
    pub fn compare_ansi(&self, after: &Style) -> String {
        let should_reset = {
            // if it used to be bold and now it's not, reset
            if self.bold.unwrap_or(false) && !after.bold.unwrap_or(false) {
                true
            }
            // if it used to be italic and now it's not, reset
            else if self.italic.unwrap_or(false) && !after.italic.unwrap_or(false) {
                true
            // if it used to be underlined and now it's not, reset
            } else if self.underlined.unwrap_or(false) && !after.underlined.unwrap_or(false) {
                true
            // if it used to be strikethrough and now it's not, reset
            } else if self.strikethrough.unwrap_or(false) && !after.strikethrough.unwrap_or(false) {
                true
            // if it used to be obfuscated and now it's not, reset
            } else if self.obfuscated.unwrap_or(false) && !after.obfuscated.unwrap_or(false) {
                true
            // if it used to have a color and now it doesn't, reset
            } else if self.color.is_some() && after.color.is_none() {
                true
            } else {
                false
            }
        };

        let mut ansi_codes = String::new();

        let before = if should_reset {
            ansi_codes.push_str("\x1b[m");
            Style::new()
        } else {
            self.clone()
        };

        // if bold used to be false/default and now it's true, set bold
        if !before.bold.unwrap_or(false) && after.bold.unwrap_or(false) {
            ansi_codes.push_str("\x1b[1m");
        }
        // if italic used to be false/default and now it's true, set italic
        if !before.italic.unwrap_or(false) && after.italic.unwrap_or(false) {
            ansi_codes.push_str("\x1b[3m");
        }
        // if underlined used to be false/default and now it's true, set underlined
        if !before.underlined.unwrap_or(false) && after.underlined.unwrap_or(false) {
            ansi_codes.push_str("\x1b[4m");
        }
        // if strikethrough used to be false/default and now it's true, set strikethrough
        if !before.strikethrough.unwrap_or(false) && after.strikethrough.unwrap_or(false) {
            ansi_codes.push_str("\x1b[9m");
        }
        // if obfuscated used to be false/default and now it's true, set obfuscated
        if !before.obfuscated.unwrap_or(false) && after.obfuscated.unwrap_or(false) {
            ansi_codes.push_str("\x1b[8m");
        }

        // if the new color is different and not none, set color
        let color_changed = {
            if before.color.is_none() && after.color.is_some() {
                true
            } else if before.color.is_some() && after.color.is_some() {
                before.color.unwrap().value != after.color.as_ref().unwrap().value
            } else {
                false
            }
        };

        if color_changed {
            let after_color = after.color.as_ref().unwrap();
            ansi_codes.push_str(&format!(
                "\x1b[38;2;{};{};{}m",
                // r
                (after_color.value >> 16) & 0xFF,
                // g
                (after_color.value >> 8) & 0xFF,
                // b
                after_color.value & 0xFF
            ));
        }

        ansi_codes
    }

    /// Apply another style to this one
    pub fn apply(&mut self, style: &Style) {
        if let Some(color) = &style.color {
            self.color = Some(color.clone());
        }
        if let Some(bold) = &style.bold {
            self.bold = Some(*bold);
        }
        if let Some(italic) = &style.italic {
            self.italic = Some(*italic);
        }
        if let Some(underlined) = &style.underlined {
            self.underlined = Some(*underlined);
        }
        if let Some(strikethrough) = &style.strikethrough {
            self.strikethrough = Some(*strikethrough);
        }
        if let Some(obfuscated) = &style.obfuscated {
            self.obfuscated = Some(*obfuscated);
        }
    }
}
