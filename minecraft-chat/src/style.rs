#[derive(Clone)]
struct TextColor {
    value: u32,
    name: Option<String>,
}

const PREFIX_CODE: char = '\u{00a7}';

struct ChatFormatting<'a> {
    name: &'a str,
    code: char,
    is_format: bool,
    id: i32,
    color: Option<u32>,
}

impl<'a> ChatFormatting<'a> {
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

// pub const BLACK: ChatFormatting = ChatFormatting::new("BLACK", '0', false, 0, Some(0));
// pub const DARK_BLUE: ChatFormatting = ChatFormatting::new("DARK_BLUE", '1', false, 1, Some(170));
// pub const DARK_GREEN: ChatFormatting =
//     ChatFormatting::new("DARK_GREEN", '2', false, 2, Some(43520));
// pub const DARK_AQUA: ChatFormatting = ChatFormatting::new("DARK_AQUA", '3', false, 3, Some(43690));
// pub const DARK_RED: ChatFormatting = ChatFormatting::new("DARK_RED", '4', false, 4, Some(1114112));
// pub const DARK_PURPLE: ChatFormatting =
//     ChatFormatting::new("DARK_PURPLE", '5', false, 5, Some(11141290));
// pub const GOLD: ChatFormatting = ChatFormatting::new("GOLD", '6', false, 6, Some(16755200));
// pub const GRAY: ChatFormatting = ChatFormatting::new("GRAY", '7', false, 7, Some(11184810));
// pub const DARK_GRAY: ChatFormatting =
//     ChatFormatting::new("DARK_GRAY", '8', false, 8, Some(5592405));
// pub const BLUE: ChatFormatting = ChatFormatting::new("BLUE", '9', false, 9, Some(5592575));
// pub const GREEN: ChatFormatting = ChatFormatting::new("GREEN", 'a', false, 10, Some(5635925));
// pub const AQUA: ChatFormatting = ChatFormatting::new("AQUA", 'b', false, 11, Some(5636095));
// pub const RED: ChatFormatting = ChatFormatting::new("RED", 'c', false, 12, Some(16733525));
// pub const LIGHT_PURPLE: ChatFormatting =
//     ChatFormatting::new("LIGHT_PURPLE", 'd', false, 13, Some(16733695));
// pub const YELLOW: ChatFormatting = ChatFormatting::new("YELLOW", 'e', false, 14, Some(16777045));
// pub const WHITE: ChatFormatting = ChatFormatting::new("WHITE", 'f', false, 15, Some(16777215));
// pub const OBFUSCATED: ChatFormatting = ChatFormatting::new("OBFUSCATED", 'k', true, -1, None);
// pub const STRIKETHROUGH: ChatFormatting = ChatFormatting::new("STRIKETHROUGH", 'm', true, -1, None);
// pub const BOLD: ChatFormatting = ChatFormatting::new("BOLD", 'l', true, -1, None);
// pub const UNDERLINE: ChatFormatting = ChatFormatting::new("UNDERLINE", 'n', true, -1, None);
// pub const ITALIC: ChatFormatting = ChatFormatting::new("ITALIC", 'o', true, -1, None);
// pub const RESET: ChatFormatting = ChatFormatting::new("RESET", 'r', true, -1, None);

impl TextColor {
    fn new(value: u32, name: Option<String>) -> Self {
        Self { value, name }
    }

    fn format(&self) -> String {
        format!("#{:06X}", self.value)
    }

    fn to_string(&self) -> String {
        if let Some(name) = &self.name {
            name.clone()
        } else {
            self.format()
        }
    }
}

#[derive(Clone)]
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
    color: Option<TextColor>,
    bold: Option<bool>,
    italic: Option<bool>,
    underlined: Option<bool>,
    strikethrough: Option<bool>,
    obfuscated: Option<bool>,
}

impl Style {
    pub fn new() -> Style {
        Style {
            color: None,
            bold: Some(false),
            italic: Some(false),
            underlined: Some(false),
            strikethrough: Some(false),
            obfuscated: Some(false),
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
}
