use std::{collections::HashMap, fmt};

#[cfg(feature = "azalea-buf")]
use azalea_buf::McBuf;
use once_cell::sync::Lazy;
use serde::{ser::SerializeStruct, Serialize, Serializer};
use serde_json::Value;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TextColor {
    pub value: u32,
    pub name: Option<String>,
}

impl Serialize for TextColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.name.is_some() {
            serializer.serialize_str(&self.name.as_ref().unwrap().to_ascii_lowercase())
        } else {
            serializer.serialize_str(&self.format())
        }
    }
}

impl TextColor {
    pub fn parse(value: String) -> Option<TextColor> {
        if value.starts_with('#') {
            let n = value.chars().skip(1).collect::<String>();
            let n = u32::from_str_radix(&n, 16).unwrap();
            return Some(TextColor::from_rgb(n));
        }
        let color_option = NAMED_COLORS.get(&value.to_ascii_uppercase());
        if let Some(color) = color_option {
            return Some(color.clone());
        }
        None
    }

    fn from_rgb(value: u32) -> TextColor {
        TextColor { value, name: None }
    }
}

static LEGACY_FORMAT_TO_COLOR: Lazy<HashMap<&'static ChatFormatting, TextColor>> =
    Lazy::new(|| {
        let mut legacy_format_to_color = HashMap::new();
        for formatter in &ChatFormatting::FORMATTERS {
            if !formatter.is_format() && *formatter != ChatFormatting::Reset {
                legacy_format_to_color.insert(
                    formatter,
                    TextColor {
                        value: formatter.color().unwrap(),
                        name: Some(formatter.name().to_string()),
                    },
                );
            }
        }
        legacy_format_to_color
    });
static NAMED_COLORS: Lazy<HashMap<String, TextColor>> = Lazy::new(|| {
    let mut named_colors = HashMap::new();
    for color in LEGACY_FORMAT_TO_COLOR.values() {
        named_colors.insert(color.name.clone().unwrap(), color.clone());
    }
    named_colors
});

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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "azalea-buf", derive(McBuf))]
pub enum ChatFormatting {
    Black,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Green,
    Aqua,
    Red,
    LightPurple,
    Yellow,
    White,
    Obfuscated,
    Strikethrough,
    Bold,
    Underline,
    Italic,
    Reset,
}

impl ChatFormatting {
    pub const FORMATTERS: [ChatFormatting; 22] = [
        ChatFormatting::Black,
        ChatFormatting::DarkBlue,
        ChatFormatting::DarkGreen,
        ChatFormatting::DarkAqua,
        ChatFormatting::DarkRed,
        ChatFormatting::DarkPurple,
        ChatFormatting::Gold,
        ChatFormatting::Gray,
        ChatFormatting::DarkGray,
        ChatFormatting::Blue,
        ChatFormatting::Green,
        ChatFormatting::Aqua,
        ChatFormatting::Red,
        ChatFormatting::LightPurple,
        ChatFormatting::Yellow,
        ChatFormatting::White,
        ChatFormatting::Obfuscated,
        ChatFormatting::Strikethrough,
        ChatFormatting::Bold,
        ChatFormatting::Underline,
        ChatFormatting::Italic,
        ChatFormatting::Reset,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            ChatFormatting::Black => "BLACK",
            ChatFormatting::DarkBlue => "DARK_BLUE",
            ChatFormatting::DarkGreen => "DARK_GREEN",
            ChatFormatting::DarkAqua => "DARK_AQUA",
            ChatFormatting::DarkRed => "DARK_RED",
            ChatFormatting::DarkPurple => "DARK_PURPLE",
            ChatFormatting::Gold => "GOLD",
            ChatFormatting::Gray => "GRAY",
            ChatFormatting::DarkGray => "DARK_GRAY",
            ChatFormatting::Blue => "BLUE",
            ChatFormatting::Green => "GREEN",
            ChatFormatting::Aqua => "AQUA",
            ChatFormatting::Red => "RED",
            ChatFormatting::LightPurple => "LIGHT_PURPLE",
            ChatFormatting::Yellow => "YELLOW",
            ChatFormatting::White => "WHITE",
            ChatFormatting::Obfuscated => "OBFUSCATED",
            ChatFormatting::Strikethrough => "STRIKETHROUGH",
            ChatFormatting::Bold => "BOLD",
            ChatFormatting::Underline => "UNDERLINE",
            ChatFormatting::Italic => "ITALIC",
            ChatFormatting::Reset => "RESET",
        }
    }

    pub fn code(&self) -> char {
        match self {
            ChatFormatting::Black => '0',
            ChatFormatting::DarkBlue => '1',
            ChatFormatting::DarkGreen => '2',
            ChatFormatting::DarkAqua => '3',
            ChatFormatting::DarkRed => '4',
            ChatFormatting::DarkPurple => '5',
            ChatFormatting::Gold => '6',
            ChatFormatting::Gray => '7',
            ChatFormatting::DarkGray => '8',
            ChatFormatting::Blue => '9',
            ChatFormatting::Green => 'a',
            ChatFormatting::Aqua => 'b',
            ChatFormatting::Red => 'c',
            ChatFormatting::LightPurple => 'd',
            ChatFormatting::Yellow => 'e',
            ChatFormatting::White => 'f',
            ChatFormatting::Obfuscated => 'k',
            ChatFormatting::Strikethrough => 'm',
            ChatFormatting::Bold => 'l',
            ChatFormatting::Underline => 'n',
            ChatFormatting::Italic => 'o',
            ChatFormatting::Reset => 'r',
        }
    }

    pub fn from_code(code: char) -> Option<ChatFormatting> {
        match code {
            '0' => Some(ChatFormatting::Black),
            '1' => Some(ChatFormatting::DarkBlue),
            '2' => Some(ChatFormatting::DarkGreen),
            '3' => Some(ChatFormatting::DarkAqua),
            '4' => Some(ChatFormatting::DarkRed),
            '5' => Some(ChatFormatting::DarkPurple),
            '6' => Some(ChatFormatting::Gold),
            '7' => Some(ChatFormatting::Gray),
            '8' => Some(ChatFormatting::DarkGray),
            '9' => Some(ChatFormatting::Blue),
            'a' => Some(ChatFormatting::Green),
            'b' => Some(ChatFormatting::Aqua),
            'c' => Some(ChatFormatting::Red),
            'd' => Some(ChatFormatting::LightPurple),
            'e' => Some(ChatFormatting::Yellow),
            'f' => Some(ChatFormatting::White),
            'k' => Some(ChatFormatting::Obfuscated),
            'm' => Some(ChatFormatting::Strikethrough),
            'l' => Some(ChatFormatting::Bold),
            'n' => Some(ChatFormatting::Underline),
            'o' => Some(ChatFormatting::Italic),
            'r' => Some(ChatFormatting::Reset),
            _ => None,
        }
    }

    pub fn is_format(&self) -> bool {
        matches!(
            self,
            ChatFormatting::Obfuscated
                | ChatFormatting::Strikethrough
                | ChatFormatting::Bold
                | ChatFormatting::Underline
                | ChatFormatting::Italic
                | ChatFormatting::Reset
        )
    }

    pub fn color(&self) -> Option<u32> {
        match self {
            ChatFormatting::Black => Some(0),
            ChatFormatting::DarkBlue => Some(170),
            ChatFormatting::DarkGreen => Some(43520),
            ChatFormatting::DarkAqua => Some(43690),
            ChatFormatting::DarkRed => Some(1114112),
            ChatFormatting::DarkPurple => Some(11141290),
            ChatFormatting::Gold => Some(16755200),
            ChatFormatting::Gray => Some(11184810),
            ChatFormatting::DarkGray => Some(5592405),
            ChatFormatting::Blue => Some(5592575),
            ChatFormatting::Green => Some(5635925),
            ChatFormatting::Aqua => Some(5636095),
            ChatFormatting::Red => Some(16733525),
            ChatFormatting::LightPurple => Some(16733695),
            ChatFormatting::Yellow => Some(16777045),
            ChatFormatting::White => Some(16777215),
            _ => None,
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
impl TryFrom<ChatFormatting> for TextColor {
    type Error = String;

    fn try_from(formatter: ChatFormatting) -> Result<Self, Self::Error> {
        if formatter.is_format() {
            return Err(format!("{} is not a color", formatter.name()));
        }
        let color = formatter.color().unwrap_or(0);
        Ok(Self::new(color, Some(formatter.name().to_string())))
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Style {
    // These are options instead of just bools because None is different than false in this case
    pub color: Option<TextColor>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underlined: Option<bool>,
    pub strikethrough: Option<bool>,
    pub obfuscated: Option<bool>,
    /// Whether formatting should be reset before applying these styles
    pub reset: bool,
}

impl Serialize for Style {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = if self.reset {
            6
        } else {
            usize::from(self.color.is_some())
                + usize::from(self.bold.is_some())
                + usize::from(self.italic.is_some())
                + usize::from(self.underlined.is_some())
                + usize::from(self.strikethrough.is_some())
                + usize::from(self.obfuscated.is_some())
        };
        let mut state = serializer.serialize_struct("Style", len)?;
        if let Some(color) = &self.color {
            state.serialize_field("color", color)?;
        } else if self.reset {
            state.serialize_field("color", "white")?;
        }
        if let Some(bold) = &self.bold {
            state.serialize_field("bold", bold)?;
        } else if self.reset {
            state.serialize_field("bold", &false)?;
        }
        if let Some(italic) = &self.italic {
            state.serialize_field("italic", italic)?;
        } else if self.reset {
            state.serialize_field("italic", &false)?;
        }
        if let Some(underlined) = &self.underlined {
            state.serialize_field("underlined", underlined)?;
        } else if self.reset {
            state.serialize_field("underlined", &false)?;
        }
        if let Some(strikethrough) = &self.strikethrough {
            state.serialize_field("strikethrough", strikethrough)?;
        } else if self.reset {
            state.serialize_field("strikethrough", &false)?;
        }
        if let Some(obfuscated) = &self.obfuscated {
            state.serialize_field("obfuscated", obfuscated)?;
        } else if self.reset {
            state.serialize_field("obfuscated", &false)?;
        }
        state.end()
    }
}

impl Style {
    pub fn empty() -> Self {
        Self::default()
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
                .and_then(|v| TextColor::parse(v.to_string()));
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
        // if strikethrough used to be false/default and now it's true, set
        // strikethrough
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
            ChatFormatting::Bold => self.bold = Some(true),
            ChatFormatting::Italic => self.italic = Some(true),
            ChatFormatting::Underline => self.underlined = Some(true),
            ChatFormatting::Strikethrough => self.strikethrough = Some(true),
            ChatFormatting::Obfuscated => self.obfuscated = Some(true),
            ChatFormatting::Reset => self.reset = true,
            formatter => {
                // if it's a color, set it
                if let Some(color) = formatter.color() {
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
                white = Ansi::rgb(ChatFormatting::White.color().unwrap()),
                italic = Ansi::ITALIC
            )
        )
    }

    #[test]
    fn test_from_code() {
        assert_eq!(
            ChatFormatting::from_code('a').unwrap(),
            ChatFormatting::Green
        );
    }

    #[test]
    fn test_apply_formatting() {
        let mut style = Style::default();
        style.apply_formatting(&ChatFormatting::Bold);
        style.apply_formatting(&ChatFormatting::Red);
        assert_eq!(style.color, Some(TextColor::from_rgb(16733525)));
    }
}
