#[cfg(all(feature = "azalea-buf", feature = "simdnbt"))]
use std::io::{self, Cursor, Write};
use std::{
    fmt::{self, Display},
    sync::LazyLock,
};

#[cfg(all(feature = "azalea-buf", feature = "simdnbt"))]
use azalea_buf::{AzBuf, BufReadError};
use azalea_codec::Codec;
#[cfg(all(feature = "azalea-buf", feature = "simdnbt"))]
use simdnbt::owned::NbtTag;

use crate::{
    base_component::BaseComponent,
    style::{ChatFormatting, Style},
    text_component::TextComponent,
    translatable_component::TranslatableComponent,
};

/// A chat component, basically anything you can see in chat.
#[derive(Clone, Debug, PartialEq)]
pub enum FormattedText {
    Text(TextComponent),
    Translatable(TranslatableComponent),
}

impl<O: azalea_codec::Ops> azalea_codec::Codec<O> for FormattedText {
    type Incomplete = azalea_codec::error::ReadResult<Self>;
    fn codec_read(ops: &O) -> azalea_codec::error::ReadResult<Self> {
        let mut errors = Vec::new();
        match String::codec_read(ops) {
            Ok(r) => {
                return Ok(Self::Text(TextComponent::new(r)));
            }
            Err(e) => {
                errors.push(("String", e));
            }
        }
        match TextComponent::codec_read(ops) {
            Ok(r) => {
                return Ok(Self::Text(r));
            }
            Err(e) => {
                errors.push(("TextComponent", e));
            }
        }
        match TranslatableComponent::codec_read(ops) {
            Ok(r) => {
                return Ok(Self::Translatable(r));
            }
            Err(e) => {
                errors.push(("TranslatableComponent", e));
            }
        }

        match Vec::<FormattedText>::codec_read(ops) {
            Ok(r) => {
                let mut iter = r.into_iter();
                let Some(mut first) = iter.next() else {
                    // an empty array is parsed as an empty string component
                    return Ok(Self::default());
                };
                // if first.get_base().siblings.is_empty() {
                //     // add the rest of the components as siblings on the
                // first     // component
                //     first.extend(iter);
                // } else {
                let style = first.get_base().style.clone();

                first = TextComponent::empty_with_siblings(vec![first])
                    .with_additional_siblings(iter)
                    .with_style(*style)
                    .into();
                // }
                return Ok(first);
            }
            Err(e) => {
                errors.push(("String", e));
            }
        }

        Err(azalea_codec::error::ReadError::no_variants_matched(errors))
    }
    fn codec_write(&self) -> ::azalea_codec::error::WriteResult<O> {
        match self {
            Self::Text(v) => {
                if v.base.is_empty() {
                    return v.text.codec_write();
                }
                v.codec_write()
            }
            Self::Translatable(v) => v.codec_write(),
        }
    }
}

pub static DEFAULT_STYLE: LazyLock<Style> = LazyLock::new(|| Style {
    color: Some(ChatFormatting::White.try_into().unwrap()),
    ..Style::default()
});

/// A chat component
impl FormattedText {
    pub fn get_base_mut(&mut self) -> &mut BaseComponent {
        match self {
            Self::Text(c) => &mut c.base,
            Self::Translatable(c) => &mut c.base,
        }
    }

    pub fn get_base(&self) -> &BaseComponent {
        match self {
            Self::Text(c) => &c.base,
            Self::Translatable(c) => &c.base,
        }
    }

    /// Add a component as a sibling of this one.
    pub fn append(&mut self, sibling: FormattedText) {
        self.extend([sibling]);
    }
    pub fn extend(&mut self, siblings: impl IntoIterator<Item = FormattedText>) {
        self.get_base_mut().siblings.extend(siblings);
    }

    /// Render all components into a single `String`, using your custom
    /// closures to drive styling, text transformation, and final cleanup.
    ///
    /// # Type parameters
    ///
    /// - `F`: `(running, component, default) -> (prefix, suffix)` for
    ///   per-component styling
    /// - `S`: `&str -> String` for text tweaks (escaping, mapping, etc.)
    /// - `C`: `&final_running_style -> String` for any trailing cleanup
    ///
    /// # Arguments
    ///
    /// - `style_formatter`: how to open/close each component's style
    /// - `text_formatter`: how to turn raw text into output text
    /// - `cleanup_formatter`: emit after all components (e.g. reset codes)
    /// - `default_style`: where to reset when a component's `reset` is true
    ///
    /// # Example
    ///
    /// ```rust
    /// use azalea_chat::{FormattedText, DEFAULT_STYLE};
    /// use serde::de::Deserialize;
    ///
    /// let component = FormattedText::deserialize(&serde_json::json!({
    ///    "text": "Hello, world!",
    ///    "color": "red",
    /// })).unwrap();
    ///
    /// let ansi = component.to_custom_format(
    ///     |running, new| (running.compare_ansi(new), "".to_owned()),
    ///     |text| text.to_string(),
    ///     |style| {
    ///         if !style.is_empty() {
    ///             "\u{1b}[m".to_owned()
    ///         } else {
    ///             "".to_owned()
    ///         }
    ///     },
    ///     &DEFAULT_STYLE,
    /// );
    /// println!("{ansi}");
    /// ```
    pub fn to_custom_format<F, S, C>(
        &self,
        mut style_formatter: F,
        mut text_formatter: S,
        mut cleanup_formatter: C,
        default_style: &Style,
    ) -> String
    where
        F: FnMut(&Style, &Style) -> (String, String),
        S: FnMut(&str) -> String,
        C: FnMut(&Style) -> String,
    {
        let mut output = String::new();

        let mut running_style = Style::default();
        self.clone().to_custom_format_recursive(
            &mut output,
            &mut style_formatter,
            &mut text_formatter,
            &default_style.clone(),
            &mut running_style,
        );
        output.push_str(&cleanup_formatter(&running_style));

        output
    }

    fn to_custom_format_recursive<F, S>(
        self,
        output: &mut String,
        style_formatter: &mut F,
        text_formatter: &mut S,
        parent_style: &Style,
        last_style: &mut Style,
    ) where
        F: FnMut(&Style, &Style) -> (String, String),
        S: FnMut(&str) -> String,
    {
        let mut expected_style = parent_style.clone();

        // convert it into a TextComponent
        let c = match self {
            FormattedText::Text(c) => c,
            FormattedText::Translatable(c) => c.into_text_component().unwrap_or_default(),
        };

        let text = &c.text;
        expected_style = expected_style.merged_with(&c.base.style);

        if !text.is_empty() {
            let (formatted_style_prefix, formatted_style_suffix) =
                style_formatter(&last_style, &expected_style);
            let formatted_text = text_formatter(text);

            output.push_str(&formatted_style_prefix);
            output.push_str(&formatted_text);
            output.push_str(&formatted_style_suffix);

            *last_style = expected_style.clone();
        }

        for sibling in c.base.siblings {
            sibling.to_custom_format_recursive(
                output,
                style_formatter,
                text_formatter,
                &expected_style,
                last_style,
            )
        }
    }

    /// Convert this component into an
    /// [ANSI string](https://en.wikipedia.org/wiki/ANSI_escape_code).
    ///
    /// This is the same as [`FormattedText::to_ansi`], but you can specify a
    /// default [`Style`] to use.
    pub fn to_ansi_with_custom_style(&self, default_style: &Style) -> String {
        self.to_custom_format(
            |running, new| (running.compare_ansi(new), "".to_owned()),
            |text| text.to_owned(),
            |style| if !style.is_empty() { "\x1b[m" } else { "" }.to_owned(),
            default_style,
        )
    }

    /// Convert this component into an
    /// [ANSI string](https://en.wikipedia.org/wiki/ANSI_escape_code), so you
    /// can print it to your terminal and get styling.
    ///
    /// This is technically a shortcut for
    /// [`FormattedText::to_ansi_with_custom_style`] with a default [`Style`]
    /// colored white.
    ///
    /// If you don't want the result to be styled at all, use
    /// [`Self::to_string`](#method.fmt-1).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use azalea_chat::FormattedText;
    /// use serde::de::Deserialize;
    ///
    /// let component = FormattedText::deserialize(&serde_json::json!({
    ///    "text": "Hello, world!",
    ///    "color": "red",
    /// })).unwrap();
    ///
    /// println!("{}", component.to_ansi());
    /// ```
    pub fn to_ansi(&self) -> String {
        self.to_ansi_with_custom_style(&DEFAULT_STYLE)
    }

    /// Similar to [`Self::to_ansi`] but renders the result as HTML instead.
    pub fn to_html(&self) -> String {
        self.to_custom_format(
            |running, new| {
                (
                    format!(
                        "<span style=\"{}\">",
                        running.merged_with(new).get_html_style()
                    ),
                    "</span>".to_owned(),
                )
            },
            |text| {
                text.replace("&", "&amp;")
                    .replace("<", "&lt;")
                    // usually unnecessary but good for compatibility
                    .replace(">", "&gt;")
                    .replace("\n", "<br>")
            },
            |_| "".to_owned(),
            &DEFAULT_STYLE,
        )
    }
}

impl IntoIterator for FormattedText {
    type Item = FormattedText;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    /// Recursively call the function for every component in this component
    fn into_iter(self) -> Self::IntoIter {
        let base = self.get_base();
        let siblings = base.siblings.clone();
        let mut v: Vec<FormattedText> = Vec::with_capacity(siblings.len() + 1);
        v.push(self);
        for sibling in siblings {
            v.extend(sibling);
        }

        v.into_iter()
    }
}

#[cfg(all(feature = "azalea-buf", feature = "simdnbt"))]
impl AzBuf for FormattedText {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let nbt = simdnbt::borrow::read_optional_tag(buf)?;
        // trace!(
        //     "Reading NBT for FormattedText: {:?}",
        //     nbt.as_ref().map(|n| n.as_tag().to_owned())
        // );

        match nbt {
            Some(nbt) => FormattedText::codec_read(&nbt.as_tag().to_owned())
                .map_err(|_| BufReadError::custom("couldn't convert nbt to chat message")),
            _ => Ok(FormattedText::default()),
        }
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        let nbt: NbtTag = self.codec_write().map_err(io::Error::other)?;
        nbt.azalea_write(buf)?;
        Ok(())
    }
}

impl From<String> for FormattedText {
    fn from(s: String) -> Self {
        FormattedText::Text(TextComponent {
            text: s,
            base: BaseComponent::default(),
        })
    }
}
impl From<&str> for FormattedText {
    fn from(s: &str) -> Self {
        Self::from(s.to_owned())
    }
}

impl Display for FormattedText {
    /// Render the text in the component but without any formatting/styling.
    ///
    /// If you want the text to be styled, consider using [`Self::to_ansi`] or
    /// [`Self::to_html`].
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.to_custom_format(
            |_, _| ("".to_owned(), "".to_owned()),
            |text| text.to_owned(),
            |_| "".to_owned(),
            &Style::default(),
        );
        f.write_str(&s)
    }
}

impl Default for FormattedText {
    fn default() -> Self {
        FormattedText::Text(TextComponent::default())
    }
}

impl serde::Serialize for FormattedText {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let json: serde_json::Value = self.codec_write().map_err(serde::ser::Error::custom)?;
        json.serialize(serializer)
    }
}
impl<'de> serde::Deserialize<'de> for FormattedText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let json = serde_json::Value::deserialize(deserializer)?;
        Self::codec_read(&json).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_json::Value;

    use super::*;
    use crate::{style::TextColor, translatable_component::PrimitiveOrComponent};

    #[test]
    fn deserialize_translation() {
        let j: Value =
            serde_json::from_str(r#"{"translate": "translation.test.args", "with": ["a", "b"]}"#)
                .unwrap();
        let component = FormattedText::codec_read(&j).unwrap();
        assert_eq!(
            component,
            FormattedText::Translatable(TranslatableComponent::new(
                "translation.test.args".to_owned(),
                vec![
                    PrimitiveOrComponent::String("a".to_owned()),
                    PrimitiveOrComponent::String("b".to_owned())
                ]
            ))
        );
    }

    #[test]
    fn deserialize_translation_invalid_arguments() {
        let j: Value =
            serde_json::from_str(r#"{"translate": "translation.test.args", "with": {}}"#).unwrap();
        assert!(FormattedText::codec_read(&j).is_err());
    }

    #[test]
    fn deserialize_translation_fallback() {
        let j: Value = serde_json::from_str(r#"{"translate": "translation.test.undefined", "fallback": "fallback: %s", "with": ["a"]}"#).unwrap();
        let component = FormattedText::codec_read(&j).unwrap();
        assert_eq!(
            component,
            FormattedText::Translatable(TranslatableComponent::with_fallback(
                "translation.test.undefined".to_owned(),
                Some("fallback: %s".to_owned()),
                vec![PrimitiveOrComponent::String("a".to_owned())]
            ))
        );
    }

    #[test]
    fn deserialize_translation_invalid_fallback() {
        let j: Value = serde_json::from_str(
            r#"{"translate": "translation.test.undefined", "fallback": {"text": "invalid"}}"#,
        )
        .unwrap();
        assert!(FormattedText::codec_read(&j).is_err());
    }
    #[test]
    fn deserialize_translation_primitive_args() {
        let j: Value = serde_json::from_str(
            r#"{"translate":"commands.list.players", "with": [1, 65536, "<players>", {"text": "unused", "color": "red"}]}"#,
        )
        .unwrap();
        assert_eq!(
            FormattedText::deserialize(&j).unwrap(),
            FormattedText::Translatable(TranslatableComponent::new(
                "commands.list.players".to_owned(),
                vec![
                    PrimitiveOrComponent::Short(1),
                    PrimitiveOrComponent::Integer(65536),
                    PrimitiveOrComponent::String("<players>".to_owned()),
                    PrimitiveOrComponent::FormattedText(FormattedText::Text(
                        TextComponent::new("unused")
                            .with_style(Style::new().color(Some(TextColor::parse("red").unwrap())))
                    ))
                ]
            ))
        );
    }

    #[test]
    fn test_translatable_with_color_inheritance() {
        let json = serde_json::json!({
            "translate": "advancements.story.smelt_iron.title",
            "color": "green",
            "with": [{"text": "Acquire Hardware"}]
        });
        let component = FormattedText::codec_read(&json).unwrap();
        let ansi = component.to_ansi();
        println!("{ansi}");
        assert!(ansi.contains("\u{1b}[38;2;85;255;85m"));
    }
}
