use std::{
    fmt::{self, Display},
    mem,
};

use azalea_codec::Codec;

use crate::{
    FormattedText,
    component::base::BaseComponent,
    style::{ChatFormatting, Style, TextColor},
};

/// A component that contains text that's the same in all locales.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TextComponent {
    pub text: String,
    pub base: BaseComponent,
}

#[derive(Clone, Debug, Default, PartialEq, Codec)]
struct TextComponentCodec {
    text: String,
    #[codec(flatten)]
    base: BaseComponent,
}
impl<O: azalea_codec::Ops> Codec<O> for TextComponent {
    fn codec_read(ops: &O, ctx: azalea_codec::Ctx<O>) -> azalea_codec::error::ReadResult<Self> {
        let r = TextComponentCodec::codec_read(ops, ctx)?;

        // make sure it goes through the constructor so legacy formatting is
        // handled
        let legacy_formatted = Self::new(r.text);

        if legacy_formatted.base.is_default() {
            return Ok(legacy_formatted.with_base(r.base));
        }

        // this will happen if legacy formatting is being mixed with modern
        // formatting. in this case, we want to wrap it like `{text: "", color:
        // etc, extra: [legacy, ...siblings]}`

        let mut siblings = vec![legacy_formatted.into()];
        siblings.extend(r.base.siblings);

        Ok(TextComponent::empty_with_siblings(siblings).with_style(*r.base.style))
    }

    fn codec_write(&self, ctx: azalea_codec::Ctx<O>) -> azalea_codec::error::WriteResult<O> {
        TextComponentCodec {
            text: self.text.clone(),
            base: self.base.clone(),
        }
        .codec_write(ctx)
    }
}

pub const LEGACY_FORMATTING_CODE_SYMBOL: char = '§';

/// Convert a legacy color code string into a FormattedText
/// Technically in Minecraft this is done when displaying the text, but AFAIK
/// it's the same as just doing it in TextComponent
pub fn legacy_color_code_to_text_component(legacy_color_code: &str) -> TextComponent {
    if legacy_color_code.is_empty() {
        return TextComponent::new("");
    }

    // only used when resetting
    let mut sections: Vec<TextComponent> = Vec::with_capacity(1);

    let mut components: Vec<TextComponent> = Vec::with_capacity(1);
    // iterate over legacy_color_code, if it starts with
    // LEGACY_COLOR_CODE_SYMBOL then read the next character and get the
    // style from that otherwise, add the character to the text

    let mut cur_component = TextComponent::new("");

    // we don't use a normal for loop since we need to be able to skip after
    // reading the formatter code symbol
    let mut i = 0;
    while i < legacy_color_code.chars().count() {
        if legacy_color_code.chars().nth(i).unwrap() == LEGACY_FORMATTING_CODE_SYMBOL {
            let formatting_code = legacy_color_code.chars().nth(i + 1);
            let Some(formatting_code) = formatting_code else {
                i += 1;
                continue;
            };
            if formatting_code == '#' {
                let color = legacy_color_code
                    .chars()
                    .skip(i + 1)
                    // 7 to include the #
                    .take(7)
                    .collect::<String>();

                if !cur_component.text.is_empty() {
                    // we need to split this into a new component
                    components.push(cur_component.clone());
                    cur_component.text = "".to_owned();
                };
                cur_component.base.style.color = TextColor::parse(&color);

                i += 6;
            } else if let Some(formatter) = ChatFormatting::from_code(formatting_code) {
                if !cur_component.text.is_empty() {
                    // we need to split this into a new component
                    components.push(cur_component.clone());
                    cur_component.text = "".to_owned();
                }
                if formatter == ChatFormatting::Reset {
                    sections.push(TextComponent::empty_with_text_component_siblings(
                        mem::take(&mut components),
                    ));
                }
                cur_component.base.style.apply_formatting(&formatter);
            }
            i += 1;
        } else {
            cur_component
                .text
                .push(legacy_color_code.chars().nth(i).unwrap());
        };
        i += 1;
    }

    components.push(cur_component);
    sections.push(TextComponent::empty_with_text_component_siblings(
        components,
    ));

    if sections.len() == 1 {
        return sections.into_iter().next().unwrap();
    }

    TextComponent::empty_with_text_component_siblings(sections)
}

impl TextComponent {
    pub fn new(text: impl Into<String>) -> Self {
        let text = text.into();
        // if it contains a §, format it
        if text.contains(LEGACY_FORMATTING_CODE_SYMBOL) {
            legacy_color_code_to_text_component(&text)
        } else {
            Self {
                base: BaseComponent::new(),
                text,
            }
        }
    }
    pub fn empty_with_siblings(siblings: Vec<FormattedText>) -> Self {
        Self {
            text: "".into(),
            base: BaseComponent {
                siblings,
                ..Default::default()
            },
        }
    }
    pub fn empty_with_text_component_siblings(siblings: Vec<TextComponent>) -> Self {
        Self::empty_with_siblings(siblings.into_iter().map(|t| t.into()).collect())
    }

    /// Get the [`FormattedText`] for this text component.
    pub fn into_formatted_text(self) -> FormattedText {
        FormattedText::Text(self)
    }

    pub fn with_base(mut self, base: BaseComponent) -> Self {
        self.base = base;
        self
    }
    pub fn with_style(mut self, style: Style) -> Self {
        *self.base.style = style;
        self
    }
    pub fn with_additional_siblings(
        mut self,
        siblings: impl IntoIterator<Item = FormattedText>,
    ) -> Self {
        self.base.siblings.extend(siblings);
        self
    }
}

impl Display for TextComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        FormattedText::Text(self.clone()).fmt(f)
    }
}

impl From<TextComponent> for FormattedText {
    fn from(c: TextComponent) -> Self {
        FormattedText::Text(c)
    }
}

#[cfg(test)]
mod tests {
    use azalea_codec::Ctx;

    use super::*;
    use crate::style::Ansi;

    #[test]
    fn test_hypixel_motd_ansi() {
        let component =
            TextComponent::new("§aHypixel Network  §c[1.8-1.18]\n§b§lHAPPY HOLIDAYS".to_owned())
                .into_formatted_text();
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
    fn test_hypixel_motd_html() {
        let component =
            TextComponent::new("§aHypixel Network  §c[1.8-1.18]\n§b§lHAPPY HOLIDAYS".to_owned())
                .into_formatted_text();

        assert_eq!(
            component.to_html(),
            format!(
                "{GREEN}Hypixel Network  {END_SPAN}{RED}[1.8-1.18]<br>{END_SPAN}{BOLD_AQUA}HAPPY HOLIDAYS{END_SPAN}",
                END_SPAN = "</span>",
                GREEN = "<span style=\"color:#55FF55;\">",
                RED = "<span style=\"color:#FF5555;\">",
                BOLD_AQUA = "<span style=\"color:#55FFFF;font-weight:bold;\">",
            )
        );
    }

    #[test]
    fn test_xss_html() {
        let component = TextComponent::new("§a<b>&\n§b</b>".to_owned()).into_formatted_text();

        assert_eq!(
            component.to_html(),
            format!(
                "{GREEN}&lt;b&gt;&amp;<br>{END_SPAN}{AQUA}&lt;/b&gt;{END_SPAN}",
                END_SPAN = "</span>",
                GREEN = "<span style=\"color:#55FF55;\">",
                AQUA = "<span style=\"color:#55FFFF;\">",
            )
        );
    }

    #[test]
    fn test_legacy_color_code_to_component() {
        let component =
            TextComponent::new("§lHello §r§1w§2o§3r§4l§5d".to_owned()).into_formatted_text();

        let attempt = component.to_ansi();
        let solution = format!(
            "{BOLD}{WHITE}Hello {RESET}{DARK_BLUE}w{DARK_GREEN}o{DARK_AQUA}r{DARK_RED}l{DARK_PURPLE}d{RESET}",
            BOLD = Ansi::BOLD,
            WHITE = Ansi::rgb(ChatFormatting::White.color().unwrap()),
            RESET = Ansi::RESET,
            DARK_BLUE = Ansi::rgb(ChatFormatting::DarkBlue.color().unwrap()),
            DARK_GREEN = Ansi::rgb(ChatFormatting::DarkGreen.color().unwrap()),
            DARK_AQUA = Ansi::rgb(ChatFormatting::DarkAqua.color().unwrap()),
            DARK_RED = Ansi::rgb(ChatFormatting::DarkRed.color().unwrap()),
            DARK_PURPLE = Ansi::rgb(ChatFormatting::DarkPurple.color().unwrap())
        );

        assert_eq!(attempt, solution);
    }

    #[test]
    fn test_legacy_color_code_with_rgb() {
        let component =
            TextComponent::new("§#Ff0000This is a test message".to_owned()).into_formatted_text();
        assert_eq!(
            component.to_ansi(),
            format!(
                "{RED}This is a test message{RESET}",
                RED = Ansi::rgb(0xff0000),
                RESET = Ansi::RESET
            )
        );
    }

    #[test]
    fn test_serialize_to_json() {
        let component = TextComponent::new("Hello §aworld".to_owned()).into_formatted_text();

        let value: serde_json::Value = component.codec_write(Ctx::new()).unwrap();

        assert_eq!(
            serde_json::to_string(&value).unwrap(),
            "{\"text\":\"\",\"extra\":[\"Hello \",{\"text\":\"world\",\"color\":\"#55FF55\"}]}"
        );
    }
}
