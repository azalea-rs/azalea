use std::{collections::HashMap, fmt};

use serde_json::Value;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TextColor {
    pub value: u32,
    pub name: Option<String>,
}

impl TextColor {
    pub fn parse(value: String) -> Result<TextColor, String> {
        if value.starts_with('#') {
            let n = value.chars().skip(1).collect::<String>();
            let n = u32::from_str_radix(&n, 16).unwrap();
            return Ok(TextColor::from_rgb(n));
        }
        let color_option = NAMED_COLORS.get(&value.to_ascii_uppercase());
        if let Some(color) = color_option {
            return Ok(color.clone());
        }
        Err(format!("Invalid color {}", value))
    }

    fn from_rgb(value: u32) -> TextColor {
        TextColor { value, name: None }
    }
}

lazy_static! {
    static ref LEGACY_FORMAT_TO_COLOR: HashMap<&'static ChatFormatting<'static>, TextColor> = {
        let mut legacy_format_to_color = HashMap::new();
        for formatter in &ChatFormatting::FORMATTERS {
            if !formatter.is_format && *formatter != ChatFormatting::RESET {
                legacy_format_to_color.insert(
                    formatter,
                    TextColor {
                        value: formatter.color.unwrap(),
                        name: Some(formatter.name.to_string()),
                    },
                );
            }
        }
        legacy_format_to_color
    };
    static ref NAMED_COLORS: HashMap<String, TextColor> = {
        let mut named_colors = HashMap::new();
        for color in LEGACY_FORMAT_TO_COLOR.values() {
            named_colors.insert(color.name.clone().unwrap(), color.clone());
        }
        named_colors
    };
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ChatFormatting<'a> {
    pub name: &'a str,
    pub code: char,
    pub is_format: bool,
    pub id: i32,
    pub color: Option<u32>,
}

pub struct Ansi {}
impl Ansi {
    pub const BOLD: &'static str = "\u{1b}[1m";
    pub const ITALIC: &'static str = "\u{1b}[3m";
    pub const UNDERLINED: &'static str = "\u{1b}[4m";
    pub const STRIKETHROUGH: &'static str = "\u{1b}[9m";
    pub const OBFUSCATED: &'static str = "\u{1b}[8m";
    pub const RESET: &'static str = "\u{1b}[m";

    pub fn rgb(value: u32) -> String {
        format!(
            "\u{1b}[38;2;{};{};{}m",
            (value >> 16) & 0xFF,
            (value >> 8) & 0xFF,
            value & 0xFF
        )
    }
}

impl<'a> ChatFormatting<'a> {
    pub const BLACK: ChatFormatting<'a> = ChatFormatting::new("BLACK", '0', false, 0, Some(0));
    pub const DARK_BLUE: ChatFormatting<'a> =
        ChatFormatting::new("DARK_BLUE", '1', false, 1, Some(170));
    pub const DARK_GREEN: ChatFormatting<'a> =
        ChatFormatting::new("DARK_GREEN", '2', false, 2, Some(43520));
    pub const DARK_AQUA: ChatFormatting<'a> =
        ChatFormatting::new("DARK_AQUA", '3', false, 3, Some(43690));
    pub const DARK_RED: ChatFormatting<'a> =
        ChatFormatting::new("DARK_RED", '4', false, 4, Some(1114112));
    pub const DARK_PURPLE: ChatFormatting<'a> =
        ChatFormatting::new("DARK_PURPLE", '5', false, 5, Some(11141290));
    pub const GOLD: ChatFormatting<'a> = ChatFormatting::new("GOLD", '6', false, 6, Some(16755200));
    pub const GRAY: ChatFormatting<'a> = ChatFormatting::new("GRAY", '7', false, 7, Some(11184810));
    pub const DARK_GRAY: ChatFormatting<'a> =
        ChatFormatting::new("DARK_GRAY", '8', false, 8, Some(5592405));
    pub const BLUE: ChatFormatting<'a> = ChatFormatting::new("BLUE", '9', false, 9, Some(5592575));
    pub const GREEN: ChatFormatting<'a> =
        ChatFormatting::new("GREEN", 'a', false, 10, Some(5635925));
    pub const AQUA: ChatFormatting<'a> = ChatFormatting::new("AQUA", 'b', false, 11, Some(5636095));
    pub const RED: ChatFormatting<'a> = ChatFormatting::new("RED", 'c', false, 12, Some(16733525));
    pub const LIGHT_PURPLE: ChatFormatting<'a> =
        ChatFormatting::new("LIGHT_PURPLE", 'd', false, 13, Some(16733695));
    pub const YELLOW: ChatFormatting<'a> =
        ChatFormatting::new("YELLOW", 'e', false, 14, Some(16777045));
    pub const WHITE: ChatFormatting<'a> =
        ChatFormatting::new("WHITE", 'f', false, 15, Some(16777215));
    pub const OBFUSCATED: ChatFormatting<'a> =
        ChatFormatting::new("OBFUSCATED", 'k', true, -1, None);
    pub const STRIKETHROUGH: ChatFormatting<'a> =
        ChatFormatting::new("STRIKETHROUGH", 'm', true, -1, None);
    pub const BOLD: ChatFormatting<'a> = ChatFormatting::new("BOLD", 'l', true, -1, None);
    pub const UNDERLINE: ChatFormatting<'a> = ChatFormatting::new("UNDERLINE", 'n', true, -1, None);
    pub const ITALIC: ChatFormatting<'a> = ChatFormatting::new("ITALIC", 'o', true, -1, None);
    pub const RESET: ChatFormatting<'a> = ChatFormatting::new("RESET", 'r', true, -1, None);

    pub const FORMATTERS: [ChatFormatting<'a>; 22] = [
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
        ChatFormatting::OBFUSCATED,
        ChatFormatting::STRIKETHROUGH,
        ChatFormatting::BOLD,
        ChatFormatting::UNDERLINE,
        ChatFormatting::ITALIC,
        ChatFormatting::RESET,
    ];

    const fn new(
        name: &str,
        code: char,
        is_format: bool,
        id: i32,
        color: Option<u32>,
    ) -> ChatFormatting {
        ChatFormatting {
            name,
            code,
            is_format,
            id,
            color,
        }
    }

    pub fn from_code(code: char) -> Result<&'static ChatFormatting<'static>, String> {
        for formatter in &ChatFormatting::FORMATTERS {
            if formatter.code == code {
                return Ok(formatter);
            }
        }
        Err(format!("Invalid formatting code {}", code))
    }
}

impl TextColor {
    fn new(value: u32, name: Option<String>) -> Self {
        Self { value, name }
    }

    pub fn format(&self) -> String {
        format!("#{:06X}", self.value)
    }
}

impl fmt::Display for TextColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "{}", name.clone())
        } else {
            write!(f, "{}", self.format())
        }
    }
}

// from ChatFormatting to TextColor
impl TryFrom<ChatFormatting<'_>> for TextColor {
    type Error = String;

    fn try_from(formatter: ChatFormatting<'_>) -> Result<Self, Self::Error> {
        if formatter.is_format {
            return Err(format!("{} is not a color", formatter.name));
        }
        let color = formatter.color.unwrap_or(0);
        Ok(Self::new(color, Some(formatter.name.to_string())))
    }
}

#[derive(Clone, Debug)]
pub struct Style {
    // these are options instead of just bools because None is different than false in this case
    pub color: Option<TextColor>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underlined: Option<bool>,
    pub strikethrough: Option<bool>,
    pub obfuscated: Option<bool>,
    /// Whether it should reset the formatting before applying these styles
    pub reset: bool,
}

impl Style {
    pub fn default() -> Self {
        Self::empty()
    }

    pub fn empty() -> Self {
        Self {
            color: None,
            bold: None,
            italic: None,
            underlined: None,
            strikethrough: None,
            obfuscated: None,
            reset: false,
        }
    }

    pub fn deserialize(json: &Value) -> Style {
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
                ..Style::default()
            }
        } else {
            Style::default()
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
    pub fn compare_ansi(&self, after: &Style, default_style: &Style) -> String {
        let should_reset = after.reset ||
            // if it used to be bold and now it's not, reset
            (self.bold.unwrap_or(false) && !after.bold.unwrap_or(true)) ||
            // if it used to be italic and now it's not, reset
            (self.italic.unwrap_or(false) && !after.italic.unwrap_or(true)) ||
            // if it used to be underlined and now it's not, reset
            (self.underlined.unwrap_or(false) && !after.underlined.unwrap_or(true)) ||
            // if it used to be strikethrough and now it's not, reset
            (self.strikethrough.unwrap_or(false) && !after.strikethrough.unwrap_or(true)) ||
            // if it used to be obfuscated and now it's not, reset
            (self.obfuscated.unwrap_or(false) && !after.obfuscated.unwrap_or(true));

        let mut ansi_codes = String::new();

        let empty_style = Style::empty();

        let (before, after) = if should_reset {
            ansi_codes.push_str(Ansi::RESET);
            let mut updated_after = if after.reset {
                default_style.clone()
            } else {
                self.clone()
            };
            updated_after.apply(after);
            (&empty_style, updated_after)
        } else {
            (self, after.clone())
        };

        // if bold used to be false/default and now it's true, set bold
        if !before.bold.unwrap_or(false) && after.bold.unwrap_or(false) {
            ansi_codes.push_str(Ansi::BOLD);
        }
        // if italic used to be false/default and now it's true, set italic
        if !before.italic.unwrap_or(false) && after.italic.unwrap_or(false) {
            ansi_codes.push_str(Ansi::ITALIC);
        }
        // if underlined used to be false/default and now it's true, set underlined
        if !before.underlined.unwrap_or(false) && after.underlined.unwrap_or(false) {
            ansi_codes.push_str(Ansi::UNDERLINED);
        }
        // if strikethrough used to be false/default and now it's true, set strikethrough
        if !before.strikethrough.unwrap_or(false) && after.strikethrough.unwrap_or(false) {
            ansi_codes.push_str(Ansi::STRIKETHROUGH);
        }
        // if obfuscated used to be false/default and now it's true, set obfuscated
        if !before.obfuscated.unwrap_or(false) && after.obfuscated.unwrap_or(false) {
            ansi_codes.push_str(Ansi::OBFUSCATED);
        }

        // if the new color is different and not none, set color
        let color_changed = {
            if before.color.is_none() && after.color.is_some() {
                true
            } else if before.color.is_some() && after.color.is_some() {
                before.color.clone().unwrap().value != after.color.as_ref().unwrap().value
            } else {
                false
            }
        };

        if color_changed {
            let after_color = after.color.as_ref().unwrap();
            ansi_codes.push_str(&Ansi::rgb(after_color.value));
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

    /// Apply a ChatFormatting to this style
    pub fn apply_formatting(&mut self, formatting: &ChatFormatting) {
        match *formatting {
            ChatFormatting::BOLD => self.bold = Some(true),
            ChatFormatting::ITALIC => self.italic = Some(true),
            ChatFormatting::UNDERLINE => self.underlined = Some(true),
            ChatFormatting::STRIKETHROUGH => self.strikethrough = Some(true),
            ChatFormatting::OBFUSCATED => self.obfuscated = Some(true),
            ChatFormatting::RESET => self.reset = true,
            ChatFormatting {
                name: _,
                code: _,
                is_format: _,
                id: _,
                color,
            } => {
                // if it's a color, set it
                if let Some(color) = color {
                    self.color = Some(TextColor::from_rgb(color));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::component::DEFAULT_STYLE;

    use super::*;

    #[test]
    fn text_color_named_colors() {
        assert_eq!(TextColor::parse("red".to_string()).unwrap().value, 16733525);
    }
    #[test]
    fn text_color_hex_colors() {
        assert_eq!(
            TextColor::parse("#a1b2c3".to_string()).unwrap().value,
            10597059
        );
    }

    #[test]
    fn ansi_difference_should_reset() {
        let style_a = Style {
            bold: Some(true),
            italic: Some(true),
            ..Style::default()
        };
        let style_b = Style {
            bold: Some(false),
            ..Style::default()
        };
        let ansi_difference = style_a.compare_ansi(&style_b, &Style::default());
        assert_eq!(
            ansi_difference,
            format!(
                "{reset}{italic}",
                reset = Ansi::RESET,
                italic = Ansi::ITALIC
            )
        )
    }
    #[test]
    fn ansi_difference_shouldnt_reset() {
        let style_a = Style {
            bold: Some(true),
            ..Style::default()
        };
        let style_b = Style {
            italic: Some(true),
            ..Style::default()
        };
        let ansi_difference = style_a.compare_ansi(&style_b, &Style::default());
        assert_eq!(ansi_difference, Ansi::ITALIC)
    }

    #[test]
    fn ansi_difference_explicit_reset() {
        let style_a = Style {
            bold: Some(true),
            ..Style::empty()
        };
        let style_b = Style {
            italic: Some(true),
            reset: true,
            ..Style::empty()
        };
        let ansi_difference = style_a.compare_ansi(&style_b, &DEFAULT_STYLE);
        assert_eq!(
            ansi_difference,
            format!(
                "{reset}{italic}{white}",
                reset = Ansi::RESET,
                white = Ansi::rgb(ChatFormatting::WHITE.color.unwrap()),
                italic = Ansi::ITALIC
            )
        )
    }

    #[test]
    fn test_from_code() {
        assert_eq!(
            ChatFormatting::from_code('a').unwrap(),
            &ChatFormatting::GREEN
        );
    }

    #[test]
    fn test_apply_formatting() {
        let mut style = Style::default();
        style.apply_formatting(&ChatFormatting::BOLD);
        style.apply_formatting(&ChatFormatting::RED);
        assert_eq!(style.color, Some(TextColor::from_rgb(16733525)));
    }
}
