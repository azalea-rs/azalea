struct TextColor {
    value: u32,
    name: Option<String>,
}

const PREFIX_CODE: char = '\u{00a7}';

struct ChatFormatting {
    name: String,
    code: char,
    is_format: bool,
    id: i32,
    color: Option<u32>,
}

impl ChatFormatting {
    fn new(name: &str, code: char, is_format: bool, id: i32, color: Option<u32>) -> ChatFormatting {
        ChatFormatting {
            name: name.to_string(),
            code,
            is_format,
            id,
            color,
        }
    }
}

enum ChatFormatting {
    BLACK = ChatFormatting::new("BLACK", '0', false, 0, Some(0)),
    DARK_BLUE = ChatFormatting::new("DARK_BLUE", '1', false, 1, Some(170)),
    DARK_GREEN = ChatFormatting::new("DARK_GREEN", '2', false, 2, Some(43520)),
    DARK_AQUA = ChatFormatting::new("DARK_AQUA", '3', false, 3, Some(43690)),
    DARK_RED = ChatFormatting::new("DARK_RED", '4', false, 4, Some(1114112)),
    DARK_PURPLE = ChatFormatting::new("DARK_PURPLE", '5', false, 5, Some(11141290)),
    GOLD = ChatFormatting::new("GOLD", '6', false, 6, Some(16755200)),
    GRAY = ChatFormatting::new("GRAY", '7', false, 7, Some(11184810)),
    DARK_GRAY = ChatFormatting::new("DARK_GRAY", '8', false, 8, Some(5592405)),
    BLUE = ChatFormatting::new("BLUE", '9', false, 9, Some(5592575)),
    GREEN = ChatFormatting::new("GREEN", 'a', false, 10, Some(5635925)),
    AQUA = ChatFormatting::new("AQUA", 'b', false, 11, Some(5636095)),
    RED = ChatFormatting::new("RED", 'c', false, 12, Some(16733525)),
    LIGHT_PURPLE = ChatFormatting::new("LIGHT_PURPLE", 'd', false, 13, Some(16733695)),
    YELLOW = ChatFormatting::new("YELLOW", 'e', false, 14, Some(16777045)),
    WHITE = ChatFormatting::new("WHITE", 'f', false, 15, Some(16777215)),
    OBFUSCATED = ChatFormatting::new("OBFUSCATED", 'k', true, -1, None),
    BOLD = ChatFormatting::new("BOLD", 'l', true, -1, None),
    STRIKETHROUGH = ChatFormatting::new("STRIKETHROUGH", 'm', true, -1, None),
    UNDERLINE = ChatFormatting::new("UNDERLINE", 'n', true, -1, None),
    ITALIC = ChatFormatting::new("ITALIC", 'o', true, -1, None),
    RESET = ChatFormatting::new("RESET", 'r', -1, -1, None),
}

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

struct Style {
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
    color: TextColor,
    bold: bool,
    italic: bool,
    underlined: bool,
    strikethrough: bool,
    obfuscated: bool,
}
