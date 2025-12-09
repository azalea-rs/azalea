#[cfg(all(feature = "azalea-buf", feature = "simdnbt"))]
use std::io::{self, Cursor, Write};
use std::{
    fmt::{self, Display},
    sync::LazyLock,
};

#[cfg(feature = "azalea-buf")]
use azalea_buf::{AzaleaRead, AzaleaWrite, BufReadError};
use serde::{Deserialize, Deserializer, Serialize, de};
#[cfg(feature = "simdnbt")]
use simdnbt::{Deserialize as _, FromNbtTag as _, Serialize as _};
#[cfg(all(feature = "azalea-buf", feature = "simdnbt"))]
use tracing::{debug, trace, warn};

use crate::{
    base_component::BaseComponent,
    style::{ChatFormatting, Style},
    text_component::TextComponent,
    translatable_component::{PrimitiveOrComponent, TranslatableComponent},
};

/// A chat component, basically anything you can see in chat.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum FormattedText {
    Text(TextComponent),
    Translatable(TranslatableComponent),
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

    /// Add a component as a sibling of this one
    fn append(&mut self, sibling: FormattedText) {
        self.get_base_mut().siblings.push(sibling);
    }

    /// Get the "separator" component from the json
    fn parse_separator(
        json: &serde_json::Value,
    ) -> Result<Option<FormattedText>, serde_json::Error> {
        if let Some(separator) = json.get("separator") {
            return Ok(Some(FormattedText::deserialize(separator)?));
        }
        Ok(None)
    }

    #[cfg(feature = "simdnbt")]
    fn parse_separator_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Option<FormattedText> {
        if let Some(separator) = nbt.get("separator") {
            FormattedText::from_nbt_tag(separator)
        } else {
            None
        }
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
    ///     |running, new| (running.compare_ansi(new), "".to_string()),
    ///     |text| text.to_string(),
    ///     |style| {
    ///         if !style.is_empty() {
    ///             "\u{1b}[m".to_string()
    ///         } else {
    ///             "".to_string()
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
        self.to_custom_format_recursive(
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
        &self,
        output: &mut String,
        style_formatter: &mut F,
        text_formatter: &mut S,
        parent_style: &Style,
        running_style: &mut Style,
    ) where
        F: FnMut(&Style, &Style) -> (String, String),
        S: FnMut(&str) -> String,
    {
        let component_style = &self.get_base().style;
        let new_style = parent_style.merged_with(component_style);

        let mut append_text = |text: &str| {
            if !text.is_empty() {
                let (formatted_style_prefix, formatted_style_suffix) =
                    style_formatter(running_style, &new_style);
                let formatted_text = text_formatter(text);

                output.push_str(&formatted_style_prefix);
                output.push_str(&formatted_text);
                output.push_str(&formatted_style_suffix);

                *running_style = new_style.clone();
            }
        };

        match &self {
            Self::Text(c) => {
                append_text(&c.text);
            }
            Self::Translatable(c) => match c.read() {
                Ok(c) => {
                    FormattedText::Text(c).to_custom_format_recursive(
                        output,
                        style_formatter,
                        text_formatter,
                        &new_style,
                        running_style,
                    );
                }
                Err(_) => {
                    append_text(&c.key);
                }
            },
        };

        for sibling in &self.get_base().siblings {
            sibling.to_custom_format_recursive(
                output,
                style_formatter,
                text_formatter,
                &new_style,
                running_style,
            );
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
            |text| text.to_string(),
            |style| if !style.is_empty() { "\u{1b}[m" } else { "" }.to_string(),
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
            |_| "".to_string(),
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

impl<'de> Deserialize<'de> for FormattedText {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let json: serde_json::Value = serde::Deserialize::deserialize(de)?;

        // we create a component that we might add siblings to
        let mut component: FormattedText;

        // if it's primitive, make it a text component
        if !json.is_array() && !json.is_object() {
            return Ok(FormattedText::Text(TextComponent::new(
                json.as_str().unwrap_or("").to_string(),
            )));
        }
        // if it's an object, do things with { text } and stuff
        else if json.is_object() {
            if let Some(text) = json.get("text") {
                let text = text.as_str().unwrap_or("").to_string();
                component = FormattedText::Text(TextComponent::new(text));
            } else if let Some(translate) = json.get("translate") {
                let translate = translate
                    .as_str()
                    .ok_or_else(|| de::Error::custom("\"translate\" must be a string"))?
                    .into();
                let fallback = if let Some(fallback) = json.get("fallback") {
                    Some(
                        fallback
                            .as_str()
                            .ok_or_else(|| de::Error::custom("\"fallback\" must be a string"))?
                            .to_string(),
                    )
                } else {
                    None
                };
                if let Some(with) = json.get("with") {
                    let with_array = with
                        .as_array()
                        .ok_or_else(|| de::Error::custom("\"with\" must be an array"))?
                        .iter()
                        .map(|item| {
                            PrimitiveOrComponent::deserialize(item).map_err(de::Error::custom)
                        })
                        .collect::<Result<Vec<PrimitiveOrComponent>, _>>()?;

                    component = FormattedText::Translatable(TranslatableComponent::with_fallback(
                        translate, fallback, with_array,
                    ));
                } else {
                    // if it doesn't have a "with", just have the with_array be empty
                    component = FormattedText::Translatable(TranslatableComponent::with_fallback(
                        translate,
                        fallback,
                        Vec::new(),
                    ));
                }
            } else if let Some(score) = json.get("score") {
                // object = GsonHelper.getAsJsonObject(jsonObject, "score");
                if score.get("name").is_none() || score.get("objective").is_none() {
                    return Err(de::Error::missing_field(
                        "A score component needs at least a name and an objective",
                    ));
                }
                // TODO
                return Err(de::Error::custom(
                    "score text components aren't yet supported",
                ));
            } else if json.get("selector").is_some() {
                return Err(de::Error::custom(
                    "selector text components aren't yet supported",
                ));
            } else if json.get("keybind").is_some() {
                return Err(de::Error::custom(
                    "keybind text components aren't yet supported",
                ));
            } else if json.get("object").is_some() {
                return Err(de::Error::custom(
                    "object text components aren't yet supported",
                ));
            } else {
                let Some(_nbt) = json.get("nbt") else {
                    return Err(de::Error::custom(
                        format!("Don't know how to turn {json} into a FormattedText").as_str(),
                    ));
                };
                let _separator =
                    FormattedText::parse_separator(&json).map_err(de::Error::custom)?;

                let _interpret = match json.get("interpret") {
                    Some(v) => v.as_bool().ok_or(Some(false)).unwrap(),
                    None => false,
                };
                if let Some(_block) = json.get("block") {}
                return Err(de::Error::custom(
                    "nbt text components aren't yet supported",
                ));
            }
            if let Some(extra) = json.get("extra") {
                let Some(extra) = extra.as_array() else {
                    return Err(de::Error::custom("Extra isn't an array"));
                };
                if extra.is_empty() {
                    return Err(de::Error::custom("Unexpected empty array of components"));
                }
                for extra_component in extra {
                    let sibling =
                        FormattedText::deserialize(extra_component).map_err(de::Error::custom)?;
                    component.append(sibling);
                }
            }

            let style = Style::deserialize(&json);
            *component.get_base_mut().style = style;

            return Ok(component);
        }
        // ok so it's not an object, if it's an array deserialize every item
        else if !json.is_array() {
            return Err(de::Error::custom(
                format!("Don't know how to turn {json} into a FormattedText").as_str(),
            ));
        }
        let json_array = json.as_array().unwrap();
        // the first item in the array is the one that we're gonna return, the others
        // are siblings
        let mut component =
            FormattedText::deserialize(&json_array[0]).map_err(de::Error::custom)?;
        for i in 1..json_array.len() {
            component.append(
                FormattedText::deserialize(json_array.get(i).unwrap())
                    .map_err(de::Error::custom)?,
            );
        }
        Ok(component)
    }
}

#[cfg(feature = "simdnbt")]
impl simdnbt::Serialize for FormattedText {
    fn to_compound(self) -> simdnbt::owned::NbtCompound {
        match self {
            FormattedText::Text(c) => c.to_compound(),
            FormattedText::Translatable(c) => c.to_compound(),
        }
    }
}

#[cfg(feature = "simdnbt")]
impl simdnbt::FromNbtTag for FormattedText {
    fn from_nbt_tag(tag: simdnbt::borrow::NbtTag) -> Option<Self> {
        // if it's a string, return a text component with that string
        if let Some(string) = tag.string() {
            Some(FormattedText::from_nbt_string(string))
        }
        // if it's a compound, make it do things with { text } and stuff
        // simdnbt::borrow::NbtTag::Compound(compound) => {
        else if let Some(compound) = tag.compound() {
            FormattedText::from_nbt_compound(compound)
        }
        // ok so it's not a compound, if it's a list deserialize every item
        else if let Some(list) = tag.list() {
            FormattedText::from_nbt_list(list)
        } else {
            Some(FormattedText::Text(TextComponent::new("".to_owned())))
        }
    }
}

#[cfg(feature = "simdnbt")]
impl FormattedText {
    fn from_nbt_string(s: &simdnbt::Mutf8Str) -> Self {
        FormattedText::from(s)
    }
    fn from_nbt_list(list: simdnbt::borrow::NbtList) -> Option<FormattedText> {
        let mut component;
        if let Some(compounds) = list.compounds() {
            component = FormattedText::from_nbt_compound(compounds.first()?)?;
            for compound in compounds.into_iter().skip(1) {
                component.append(FormattedText::from_nbt_compound(compound)?);
            }
        } else if let Some(strings) = list.strings() {
            component = FormattedText::from(*(strings.first()?));
            for &string in strings.iter().skip(1) {
                component.append(FormattedText::from(string));
            }
        } else {
            debug!("couldn't parse {list:?} as FormattedText");
            return None;
        }
        Some(component)
    }

    pub fn from_nbt_compound(compound: simdnbt::borrow::NbtCompound) -> Option<Self> {
        let mut component: FormattedText;

        if let Some(text) = compound.get("text") {
            let text = text.string().unwrap_or_default().to_string();
            component = FormattedText::Text(TextComponent::new(text));
        } else if let Some(translate) = compound.get("translate") {
            let translate = translate.string()?.into();
            if let Some(with) = compound.get("with") {
                let mut with_array = Vec::new();
                let with_list = with.list()?;
                if with_list.empty() {
                } else if let Some(with) = with_list.strings() {
                    for item in with {
                        with_array.push(PrimitiveOrComponent::String(item.to_string()));
                    }
                } else if let Some(with) = with_list.ints() {
                    for item in with {
                        with_array.push(PrimitiveOrComponent::Integer(item));
                    }
                } else if let Some(with) = with_list.compounds() {
                    for item in with {
                        // if it's a string component with no styling and no siblings,
                        // just add a string to
                        // with_array otherwise add the
                        // component to the array
                        if let Some(primitive) = item.get("") {
                            // minecraft does this sometimes, for example
                            // for the /give system messages
                            if let Some(b) = primitive.byte() {
                                // interpreted as boolean
                                with_array.push(PrimitiveOrComponent::Boolean(b != 0));
                            } else if let Some(s) = primitive.short() {
                                with_array.push(PrimitiveOrComponent::Short(s));
                            } else if let Some(i) = primitive.int() {
                                with_array.push(PrimitiveOrComponent::Integer(i));
                            } else if let Some(l) = primitive.long() {
                                with_array.push(PrimitiveOrComponent::Long(l));
                            } else if let Some(f) = primitive.float() {
                                with_array.push(PrimitiveOrComponent::Float(f));
                            } else if let Some(d) = primitive.double() {
                                with_array.push(PrimitiveOrComponent::Double(d));
                            } else if let Some(s) = primitive.string() {
                                with_array.push(PrimitiveOrComponent::String(s.to_string()));
                            } else {
                                warn!(
                                    "couldn't parse {item:?} as FormattedText because it has a disallowed primitive"
                                );
                                with_array.push(PrimitiveOrComponent::String("?".to_string()));
                            }
                        } else if let Some(c) = FormattedText::from_nbt_compound(item) {
                            if let FormattedText::Text(text_component) = c
                                && text_component.base.siblings.is_empty()
                                && text_component.base.style.is_empty()
                            {
                                with_array.push(PrimitiveOrComponent::String(text_component.text));
                                continue;
                            }
                            with_array.push(PrimitiveOrComponent::FormattedText(
                                FormattedText::from_nbt_compound(item)?,
                            ));
                        } else {
                            warn!("couldn't parse {item:?} as FormattedText");
                            with_array.push(PrimitiveOrComponent::String("?".to_string()));
                        }
                    }
                } else {
                    warn!(
                        "couldn't parse {with:?} as FormattedText because it's not a list of compounds"
                    );
                    return None;
                }
                component =
                    FormattedText::Translatable(TranslatableComponent::new(translate, with_array));
            } else {
                // if it doesn't have a "with", just have the with_array be empty
                component =
                    FormattedText::Translatable(TranslatableComponent::new(translate, Vec::new()));
            }
        } else if let Some(score) = compound.compound("score") {
            if score.get("name").is_none() || score.get("objective").is_none() {
                trace!("A score component needs at least a name and an objective");
                return None;
            }
            // TODO: implement these
            return None;
        } else if compound.get("selector").is_some() {
            trace!("selector text components aren't supported");
            return None;
        } else if compound.get("keybind").is_some() {
            trace!("keybind text components aren't supported");
            return None;
        } else if compound.get("object").is_some() {
            trace!("object text components aren't supported");
            return None;
        } else if let Some(tag) = compound.get("") {
            return FormattedText::from_nbt_tag(tag);
        } else {
            let _nbt = compound.get("nbt")?;
            let _separator = FormattedText::parse_separator_nbt(&compound)?;

            let _interpret = match compound.get("interpret") {
                Some(v) => v.byte().unwrap_or_default() != 0,
                None => false,
            };
            if let Some(_block) = compound.get("block") {}
            trace!("nbt text components aren't yet supported");
            return None;
        }
        if let Some(extra) = compound.get("extra") {
            // if it's an array, deserialize every item
            if let Some(items) = extra.list() {
                if let Some(items) = items.compounds() {
                    for item in items {
                        component.append(FormattedText::from_nbt_compound(item)?);
                    }
                } else if let Some(items) = items.strings() {
                    for item in items {
                        component.append(FormattedText::from_nbt_string(item));
                    }
                } else if let Some(items) = items.lists() {
                    for item in items {
                        component.append(FormattedText::from_nbt_list(item)?);
                    }
                } else {
                    warn!(
                        "couldn't parse {items:?} as FormattedText because it's not a list of compounds or strings"
                    );
                }
            } else {
                component.append(FormattedText::from_nbt_tag(extra)?);
            }
        }

        let base_style = Style::from_compound(compound).ok()?;
        let new_style = &mut component.get_base_mut().style;
        **new_style = new_style.merged_with(&base_style);

        Some(component)
    }
}

#[cfg(feature = "simdnbt")]
impl From<&simdnbt::Mutf8Str> for FormattedText {
    fn from(s: &simdnbt::Mutf8Str) -> Self {
        FormattedText::Text(TextComponent::new(s.to_string()))
    }
}

#[cfg(all(feature = "azalea-buf", feature = "simdnbt"))]
impl AzaleaRead for FormattedText {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let nbt = simdnbt::borrow::read_optional_tag(buf)?;
        trace!(
            "Reading NBT for FormattedText: {:?}",
            nbt.as_ref().map(|n| n.as_tag().to_owned())
        );
        match nbt {
            Some(nbt) => FormattedText::from_nbt_tag(nbt.as_tag()).ok_or(BufReadError::Custom(
                "couldn't convert nbt to chat message".to_owned(),
            )),
            _ => Ok(FormattedText::default()),
        }
    }
}

#[cfg(all(feature = "azalea-buf", feature = "simdnbt"))]
impl AzaleaWrite for FormattedText {
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        let mut out = Vec::new();
        simdnbt::owned::BaseNbt::write_unnamed(&(self.clone().to_compound().into()), &mut out);
        buf.write_all(&out)
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
        Self::from(s.to_string())
    }
}
impl From<TranslatableComponent> for FormattedText {
    fn from(c: TranslatableComponent) -> Self {
        FormattedText::Translatable(c)
    }
}
impl From<TextComponent> for FormattedText {
    fn from(c: TextComponent) -> Self {
        FormattedText::Text(c)
    }
}

impl Display for FormattedText {
    /// Render the text in the component but without any formatting/styling.
    ///
    /// If you want the text to be styled, consider using [`Self::to_ansi`] or
    /// [`Self::to_html`].
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormattedText::Text(c) => c.fmt(f),
            FormattedText::Translatable(c) => c.fmt(f),
        }
    }
}

impl Default for FormattedText {
    fn default() -> Self {
        FormattedText::Text(TextComponent::default())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::*;
    use crate::style::TextColor;

    #[test]
    fn deserialize_translation() {
        let j: Value =
            serde_json::from_str(r#"{"translate": "translation.test.args", "with": ["a", "b"]}"#)
                .unwrap();
        let component = FormattedText::deserialize(&j).unwrap();
        assert_eq!(
            component,
            FormattedText::Translatable(TranslatableComponent::new(
                "translation.test.args".to_string(),
                vec![
                    PrimitiveOrComponent::String("a".to_string()),
                    PrimitiveOrComponent::String("b".to_string())
                ]
            ))
        );
    }

    #[test]
    fn deserialize_translation_invalid_arguments() {
        let j: Value =
            serde_json::from_str(r#"{"translate": "translation.test.args", "with": {}}"#).unwrap();
        assert!(FormattedText::deserialize(&j).is_err());
    }

    #[test]
    fn deserialize_translation_fallback() {
        let j: Value = serde_json::from_str(r#"{"translate": "translation.test.undefined", "fallback": "fallback: %s", "with": ["a"]}"#).unwrap();
        let component = FormattedText::deserialize(&j).unwrap();
        assert_eq!(
            component,
            FormattedText::Translatable(TranslatableComponent::with_fallback(
                "translation.test.undefined".to_string(),
                Some("fallback: %s".to_string()),
                vec![PrimitiveOrComponent::String("a".to_string())]
            ))
        );
    }

    #[test]
    fn deserialize_translation_invalid_fallback() {
        let j: Value = serde_json::from_str(
            r#"{"translate": "translation.test.undefined", "fallback": {"text": "invalid"}}"#,
        )
        .unwrap();
        assert!(FormattedText::deserialize(&j).is_err());
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
                "commands.list.players".to_string(),
                vec![
                    PrimitiveOrComponent::Short(1),
                    PrimitiveOrComponent::Integer(65536),
                    PrimitiveOrComponent::String("<players>".to_string()),
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
        let component = FormattedText::deserialize(&json).unwrap();
        let ansi = component.to_ansi();
        assert!(ansi.contains("\u{1b}[38;2;85;255;85m"));
    }
}
