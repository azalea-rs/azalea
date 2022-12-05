use crate::{
    base_component::BaseComponent,
    style::{ChatFormatting, Style},
    text_component::TextComponent,
    translatable_component::{StringOrComponent, TranslatableComponent},
};
use azalea_buf::{BufReadError, McBufReadable, McBufWritable};
use once_cell::sync::Lazy;
use serde::{de, Deserialize, Deserializer, Serialize};
use std::{
    fmt::Display,
    io::{Cursor, Write},
};

/// A chat component, basically anything you can see in chat.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Component {
    Text(TextComponent),
    Translatable(TranslatableComponent),
}

pub static DEFAULT_STYLE: Lazy<Style> = Lazy::new(|| Style {
    color: Some(ChatFormatting::White.try_into().unwrap()),
    ..Style::default()
});

/// A chat component
impl Component {
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
    fn append(&mut self, sibling: Component) {
        self.get_base_mut().siblings.push(sibling);
    }

    /// Get the "separator" component from the json
    fn parse_separator(json: &serde_json::Value) -> Result<Option<Component>, serde_json::Error> {
        if json.get("separator").is_some() {
            return Ok(Some(Component::deserialize(
                json.get("separator").unwrap(),
            )?));
        }
        Ok(None)
    }

    /// Convert this component into an
    /// [ANSI string](https://en.wikipedia.org/wiki/ANSI_escape_code), so you
    /// can print it to your terminal and get styling.
    ///
    /// This is technically a shortcut for [`Component::to_ansi_custom_style`] with a
    /// default [`Style`] colored white.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use azalea_chat::Component;
    /// use serde::de::Deserialize;
    ///
    /// let component = Component::deserialize(&serde_json::json!({
    ///    "text": "Hello, world!",
    ///    "color": "red",
    /// })).unwrap();
    ///
    /// println!("{}", component.to_ansi());
    /// ```
    pub fn to_ansi(&self) -> String {
        // default the default_style to white if it's not set
        self.to_ansi_custom_style(&DEFAULT_STYLE)
    }

    /// Convert this component into an
    /// [ANSI string](https://en.wikipedia.org/wiki/ANSI_escape_code).
    ///
    /// This is the same as [`Component::to_ansi`], but you can specify a
    /// default [`Style`] to use.
    pub fn to_ansi_custom_style(&self, default_style: &Style) -> String {
        // this contains the final string will all the ansi escape codes
        let mut built_string = String::new();
        // this style will update as we visit components
        let mut running_style = Style::default();

        for component in self.clone().into_iter() {
            let component_text = match &component {
                Self::Text(c) => c.text.to_string(),
                Self::Translatable(c) => c.to_string(),
            };

            let component_style = &component.get_base().style;

            let ansi_text = running_style.compare_ansi(component_style, default_style);
            built_string.push_str(&ansi_text);
            built_string.push_str(&component_text);

            running_style.apply(component_style);
        }

        if !running_style.is_empty() {
            built_string.push_str("\u{1b}[m");
        }

        built_string
    }
}

impl IntoIterator for Component {
    /// Recursively call the function for every component in this component
    fn into_iter(self) -> Self::IntoIter {
        let base = self.get_base();
        let siblings = base.siblings.clone();
        let mut v: Vec<Component> = Vec::with_capacity(siblings.len() + 1);
        v.push(self);
        for sibling in siblings {
            v.extend(sibling.into_iter());
        }

        v.into_iter()
    }

    type Item = Component;
    type IntoIter = std::vec::IntoIter<Self::Item>;
}

impl<'de> Deserialize<'de> for Component {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let json: serde_json::Value = serde::Deserialize::deserialize(de)?;

        // we create a component that we might add siblings to
        let mut component: Component;

        // if it's primitive, make it a text component
        if !json.is_array() && !json.is_object() {
            return Ok(Component::Text(TextComponent::new(
                json.as_str().unwrap_or("").to_string(),
            )));
        }
        // if it's an object, do things with { text } and stuff
        else if json.is_object() {
            if let Some(text) = json.get("text") {
                let text = text.as_str().unwrap_or("").to_string();
                component = Component::Text(TextComponent::new(text));
            } else if let Some(translate) = json.get("translate") {
                let translate = translate
                    .as_str()
                    .ok_or_else(|| de::Error::custom("\"translate\" must be a string"))?
                    .into();
                if let Some(with) = json.get("with") {
                    let with = with
                        .as_array()
                        .ok_or_else(|| de::Error::custom("\"with\" must be an array"))?;
                    let mut with_array = Vec::with_capacity(with.len());
                    for item in with {
                        // if it's a string component with no styling and no siblings, just add a string to with_array
                        // otherwise add the component to the array
                        let c = Component::deserialize(item).map_err(de::Error::custom)?;
                        if let Component::Text(text_component) = c {
                            if text_component.base.siblings.is_empty()
                                && text_component.base.style.is_empty()
                            {
                                with_array.push(StringOrComponent::String(text_component.text));
                                continue;
                            }
                        }
                        with_array.push(StringOrComponent::Component(
                            Component::deserialize(item).map_err(de::Error::custom)?,
                        ));
                    }
                    component =
                        Component::Translatable(TranslatableComponent::new(translate, with_array));
                } else {
                    // if it doesn't have a "with", just have the with_array be empty
                    component =
                        Component::Translatable(TranslatableComponent::new(translate, Vec::new()));
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
            } else {
                let _nbt = if let Some(nbt) = json.get("nbt") {
                    nbt
                } else {
                    return Err(de::Error::custom(
                        format!("Don't know how to turn {json} into a Component").as_str(),
                    ));
                };
                let _separator = Component::parse_separator(&json).map_err(de::Error::custom)?;

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
                let extra = match extra.as_array() {
                    Some(r) => r,
                    None => return Err(de::Error::custom("Extra isn't an array")),
                };
                if extra.is_empty() {
                    return Err(de::Error::custom("Unexpected empty array of components"));
                }
                for extra_component in extra {
                    let sibling =
                        Component::deserialize(extra_component).map_err(de::Error::custom)?;
                    component.append(sibling);
                }
            }

            let style = Style::deserialize(&json);
            component.get_base_mut().style = style;

            return Ok(component);
        }
        // ok so it's not an object, if it's an array deserialize every item
        else if !json.is_array() {
            return Err(de::Error::custom(
                format!("Don't know how to turn {json} into a Component").as_str(),
            ));
        }
        let json_array = json.as_array().unwrap();
        // the first item in the array is the one that we're gonna return, the others are siblings
        let mut component = Component::deserialize(&json_array[0]).map_err(de::Error::custom)?;
        for i in 1..json_array.len() {
            component.append(
                Component::deserialize(json_array.get(i).unwrap()).map_err(de::Error::custom)?,
            );
        }
        Ok(component)
    }
}

impl McBufReadable for Component {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let string = String::read_from(buf)?;
        let json: serde_json::Value = serde_json::from_str(string.as_str())?;
        let component = Component::deserialize(json)?;
        Ok(component)
    }
}

impl McBufWritable for Component {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let json = serde_json::to_string(self).unwrap();
        json.write_into(buf)?;
        Ok(())
    }
}

impl From<String> for Component {
    fn from(s: String) -> Self {
        Component::Text(TextComponent {
            text: s,
            base: BaseComponent::default(),
        })
    }
}

impl Display for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Component::Text(c) => c.fmt(f),
            Component::Translatable(c) => c.fmt(f),
        }
    }
}

impl Default for Component {
    fn default() -> Self {
        Component::Text(TextComponent::default())
    }
}
