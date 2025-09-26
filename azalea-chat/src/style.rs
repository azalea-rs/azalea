use std::{collections::HashMap, fmt, sync::LazyLock};

#[cfg(feature = "azalea-buf")]
use azalea_buf::AzBuf;
use serde::{Serialize, Serializer, ser::SerializeMap};
use serde_json::Value;
#[cfg(feature = "simdnbt")]
use simdnbt::owned::{NbtCompound, NbtTag};

use crate::{click_event::ClickEvent, hover_event::HoverEvent};

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct TextColor {
    pub value: u32,
    pub name: Option<String>,
}

impl Serialize for TextColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.serialize())
    }
}

#[cfg(feature = "simdnbt")]
impl simdnbt::ToNbtTag for TextColor {
    fn to_nbt_tag(self) -> simdnbt::owned::NbtTag {
        NbtTag::String(self.serialize().into())
    }
}

impl TextColor {
    /// Parse a text component in the same way that Minecraft does.
    ///
    /// This supports named colors and hex codes.
    pub fn parse(value: &str) -> Option<TextColor> {
        if value.starts_with('#') {
            let n = value.chars().skip(1).collect::<String>();
            let n = u32::from_str_radix(&n, 16).ok()?;
            return Some(TextColor::from_rgb(n));
        }
        let color_option = NAMED_COLORS.get(&value.to_ascii_lowercase());
        if let Some(color) = color_option {
            return Some(color.clone());
        }
        None
    }

    fn from_rgb(value: u32) -> TextColor {
        TextColor { value, name: None }
    }
}

static LEGACY_FORMAT_TO_COLOR: LazyLock<HashMap<&'static ChatFormatting, TextColor>> =
    LazyLock::new(|| {
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
static NAMED_COLORS: LazyLock<HashMap<String, TextColor>> = LazyLock::new(|| {
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
    // "Conceal or hide"
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
#[cfg_attr(feature = "azalea-buf", derive(AzBuf))]
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
            ChatFormatting::Black => "black",
            ChatFormatting::DarkBlue => "dark_blue",
            ChatFormatting::DarkGreen => "dark_green",
            ChatFormatting::DarkAqua => "dark_aqua",
            ChatFormatting::DarkRed => "dark_red",
            ChatFormatting::DarkPurple => "dark_purple",
            ChatFormatting::Gold => "gold",
            ChatFormatting::Gray => "gray",
            ChatFormatting::DarkGray => "dark_gray",
            ChatFormatting::Blue => "blue",
            ChatFormatting::Green => "green",
            ChatFormatting::Aqua => "aqua",
            ChatFormatting::Red => "red",
            ChatFormatting::LightPurple => "light_purple",
            ChatFormatting::Yellow => "yellow",
            ChatFormatting::White => "white",
            ChatFormatting::Obfuscated => "obfuscated",
            ChatFormatting::Strikethrough => "strikethrough",
            ChatFormatting::Bold => "bold",
            ChatFormatting::Underline => "underline",
            ChatFormatting::Italic => "italic",
            ChatFormatting::Reset => "reset",
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
            ChatFormatting::DarkRed => Some(11141120),
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

    fn serialize(&self) -> String {
        if let Some(name) = &self.name {
            name.clone().to_ascii_lowercase()
        } else {
            self.format_value()
        }
    }

    pub fn format_value(&self) -> String {
        format!("#{:06X}", self.value)
    }
}

impl fmt::Display for TextColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.serialize())
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

macro_rules! define_style_struct {
    ($($(#[$doc:meta])* $field:ident : $type:ty),* $(,)?) => {
        #[derive(Clone, Debug, Default, PartialEq, serde::Serialize)]
        #[non_exhaustive]
        pub struct Style {
            $(
                #[serde(skip_serializing_if = "Option::is_none")]
                $(#[$doc])*
                pub $field: Option<$type>,
            )*
        }

        impl Style {
            $(
                pub fn $field(mut self, value: impl Into<Option<$type>>) -> Self {
                    self.$field = value.into();
                    self
                }
            )*

            pub fn serialize_map<S>(&self, state: &mut S::SerializeMap) -> Result<(), S::Error>
            where
                S: serde::Serializer,
            {
                $(
                    if let Some(value) = &self.$field {
                        state.serialize_entry(stringify!($field), value)?;
                    }
                )*
                Ok(())
            }

            /// Apply another style to this one
            pub fn apply(&mut self, style: &Style) {
                $(
                    if let Some(value) = &style.$field {
                        self.$field = Some(value.clone());
                    }
                )*
            }
        }

        #[cfg(feature = "simdnbt")]
        impl simdnbt::Serialize for Style {
            fn to_compound(self) -> NbtCompound {
                let mut compound = NbtCompound::new();

                $(
                    if let Some(value) = self.$field {
                        compound.insert(stringify!($field), value);
                    }
                )*

                compound
            }
        }
    };
}

define_style_struct! {
    color: TextColor,
    shadow_color: u32,
    bold: bool,
    italic: bool,
    underlined: bool,
    strikethrough: bool,
    obfuscated: bool,
    click_event: ClickEvent,
    hover_event: HoverEvent,
    insertion: String,
    /// Represented as a `ResourceLocation`.
    font: String,
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn deserialize(json: &Value) -> Style {
        let Some(j) = json.as_object() else {
            return Style::default();
        };

        Style {
            color: j
                .get("color")
                .and_then(|v| v.as_str())
                .and_then(TextColor::parse),
            shadow_color: j
                .get("shadow_color")
                .and_then(|v| v.as_u64())
                .map(|v| v as u32),
            bold: j.get("bold").and_then(|v| v.as_bool()),
            italic: j.get("italic").and_then(|v| v.as_bool()),
            underlined: j.get("underlined").and_then(|v| v.as_bool()),
            strikethrough: j.get("strikethrough").and_then(|v| v.as_bool()),
            obfuscated: j.get("obfuscated").and_then(|v| v.as_bool()),
            // TODO: impl deserialize functions for click_event and hover_event
            click_event: Default::default(),
            hover_event: Default::default(),
            insertion: j
                .get("insertion")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            font: j
                .get("font")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
        }
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
        let should_reset =
            // if it used to be bold and now it's not, reset
            (self.bold.unwrap_or_default() && !after.bold.unwrap_or_default()) ||
            // if it used to be italic and now it's not, reset
            (self.italic.unwrap_or_default() && !after.italic.unwrap_or_default()) ||
            // if it used to be underlined and now it's not, reset
            (self.underlined.unwrap_or_default() && !after.underlined.unwrap_or_default()) ||
            // if it used to be strikethrough and now it's not, reset
            (self.strikethrough.unwrap_or_default() && !after.strikethrough.unwrap_or_default()) ||
            // if it used to be obfuscated and now it's not, reset
            (self.obfuscated.unwrap_or_default() && !after.obfuscated.unwrap_or_default());

        let mut ansi_codes = String::new();

        let empty_style = Style::empty();

        let before = if should_reset {
            ansi_codes.push_str(Ansi::RESET);
            &empty_style
        } else {
            self
        };

        // if bold used to be false/default and now it's true, set bold
        if !before.bold.unwrap_or_default() && after.bold.unwrap_or_default() {
            ansi_codes.push_str(Ansi::BOLD);
        }
        // if italic used to be false/default and now it's true, set italic
        if !before.italic.unwrap_or_default() && after.italic.unwrap_or_default() {
            ansi_codes.push_str(Ansi::ITALIC);
        }
        // if underlined used to be false/default and now it's true, set underlined
        if !before.underlined.unwrap_or_default() && after.underlined.unwrap_or_default() {
            ansi_codes.push_str(Ansi::UNDERLINED);
        }
        // if strikethrough used to be false/default and now it's true, set
        // strikethrough
        if !before.strikethrough.unwrap_or_default() && after.strikethrough.unwrap_or_default() {
            ansi_codes.push_str(Ansi::STRIKETHROUGH);
        }
        // if obfuscated used to be false/default and now it's true, set obfuscated
        if !before.obfuscated.unwrap_or_default() && after.obfuscated.unwrap_or_default() {
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

    /// Returns a new style that is a merge of self and other.
    /// For any field that `other` does not specify (is None), self’s value is
    /// used.
    pub fn merged_with(&self, other: &Style) -> Style {
        Style {
            color: other.color.clone().or(self.color.clone()),
            shadow_color: other.shadow_color.or(self.shadow_color),
            bold: other.bold.or(self.bold),
            italic: other.italic.or(self.italic),
            underlined: other.underlined.or(self.underlined),
            strikethrough: other.strikethrough.or(self.strikethrough),
            obfuscated: other.obfuscated.or(self.obfuscated),
            click_event: other.click_event.clone().or(self.click_event.clone()),
            hover_event: other.hover_event.clone().or(self.hover_event.clone()),
            insertion: other.insertion.clone().or(self.insertion.clone()),
            font: other.font.clone().or(self.font.clone()),
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
            ChatFormatting::Reset => {
                self.color = None;
                self.bold = None;
                self.italic = None;
                self.underlined = None;
                self.strikethrough = None;
                self.obfuscated = None;
            }
            formatter => {
                // if it's a color, set it
                if let Some(color) = formatter.color() {
                    self.color = Some(TextColor::from_rgb(color));
                }
            }
        }
    }

    pub fn get_html_style(&self) -> String {
        let mut style = String::new();
        if let Some(color) = &self.color {
            style.push_str(&format!("color:{};", color.format_value()));
        }
        if let Some(bold) = self.bold {
            style.push_str(&format!(
                "font-weight:{};",
                if bold { "bold" } else { "normal" }
            ));
        }
        if let Some(italic) = self.italic {
            style.push_str(&format!(
                "font-style:{};",
                if italic { "italic" } else { "normal" }
            ));
        }
        if let Some(underlined) = self.underlined {
            style.push_str(&format!(
                "text-decoration:{};",
                if underlined { "underline" } else { "none" }
            ));
        }
        if let Some(strikethrough) = self.strikethrough {
            style.push_str(&format!(
                "text-decoration:{};",
                if strikethrough {
                    "line-through"
                } else {
                    "none"
                }
            ));
        }
        if let Some(obfuscated) = self.obfuscated
            && obfuscated
        {
            style.push_str("filter:blur(2px);");
        }

        style
    }
}

#[cfg(feature = "simdnbt")]
impl simdnbt::Deserialize for Style {
    fn from_compound(
        compound: simdnbt::borrow::NbtCompound,
    ) -> Result<Self, simdnbt::DeserializeError> {
        let bold = compound.byte("bold").map(|v| v != 0);
        let italic = compound.byte("italic").map(|v| v != 0);
        let underlined = compound.byte("underlined").map(|v| v != 0);
        let strikethrough = compound.byte("strikethrough").map(|v| v != 0);
        let obfuscated = compound.byte("obfuscated").map(|v| v != 0);
        let color: Option<TextColor> = compound
            .string("color")
            .and_then(|v| TextColor::parse(&v.to_str()));
        Ok(Style {
            color,
            bold,
            italic,
            underlined,
            strikethrough,
            obfuscated,
            ..Style::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_color_named_colors() {
        assert_eq!(TextColor::parse("red").unwrap().value, 16733525);
    }
    #[test]
    fn text_color_hex_colors() {
        assert_eq!(TextColor::parse("#a1b2c3").unwrap().value, 10597059);
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
            italic: Some(true),
            ..Style::default()
        };
        let ansi_difference = style_a.compare_ansi(&style_b);
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
            bold: Some(true),
            italic: Some(true),
            ..Style::default()
        };
        let ansi_difference = style_a.compare_ansi(&style_b);
        assert_eq!(ansi_difference, Ansi::ITALIC)
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
