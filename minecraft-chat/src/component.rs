use serde::{
    de::{self, Error},
    Deserialize, Deserializer,
};

use crate::{
    base_component::BaseComponent,
    style::{ChatFormatting, Style},
    text_component::TextComponent,
    translatable_component::{StringOrComponent, TranslatableComponent},
};

#[derive(Clone, Debug)]
pub enum Component {
    Text(TextComponent),
    Translatable(TranslatableComponent),
}

lazy_static! {
    pub static ref DEFAULT_STYLE: Style = Style {
        color: Some(ChatFormatting::WHITE.try_into().unwrap()),
        ..Style::default()
    };
}

/// A chat component
impl Component {
    // TODO: is it possible to use a macro so this doesn't have to be duplicated?

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

    /// Convert this component into an ansi string
    pub fn to_ansi(&self, default_style: Option<&Style>) -> String {
        // default the default_style to white if it's not set
        let default_style: &Style = default_style.unwrap_or_else(|| &DEFAULT_STYLE);

        // this contains the final string will all the ansi escape codes
        let mut built_string = String::new();
        // this style will update as we visit components
        let mut running_style = Style::default();

        for component in self.clone().into_iter() {
            let component_text = match &component {
                Self::Text(c) => &c.text,
                Self::Translatable(c) => &c.key,
            };
            let component_style = &component.get_base().style;

            let ansi_text = running_style.compare_ansi(component_style, default_style);
            built_string.push_str(&ansi_text);
            built_string.push_str(component_text);

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
        println!("deserializing component");
        let json: serde_json::Value = serde::Deserialize::deserialize(de)?;
        println!("made json");

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
            if json.get("text").is_some() {
                let text = json.get("text").unwrap().as_str().unwrap_or("").to_string();
                component = Component::Text(TextComponent::new(text));
            } else if json.get("translate").is_some() {
                let translate = json.get("translate").unwrap().to_string();
                if json.get("with").is_some() {
                    let with = json.get("with").unwrap().as_array().unwrap();
                    let mut with_array = Vec::with_capacity(with.len());
                    for i in 0..with.len() {
                        // if it's a string component with no styling and no siblings, just add a string to with_array
                        // otherwise add the component to the array
                        let c = Component::deserialize(&with[i]).map_err(de::Error::custom)?;
                        if let Component::Text(text_component) = c {
                            if text_component.base.siblings.is_empty()
                                && text_component.base.style.is_empty()
                            {
                                with_array.push(StringOrComponent::String(text_component.text));
                                break;
                            }
                        }
                        with_array.push(StringOrComponent::Component(
                            Component::deserialize(&with[i]).map_err(de::Error::custom)?,
                        ));
                    }
                    component =
                        Component::Translatable(TranslatableComponent::new(translate, with_array));
                } else {
                    // if it doesn't have a "with", just have the with_array be empty
                    component =
                        Component::Translatable(TranslatableComponent::new(translate, Vec::new()));
                }
            } else if json.get("score").is_some() {
                // object = GsonHelper.getAsJsonObject(jsonObject, "score");
                let score_json = json.get("score").unwrap();
                //     if (!object.has("name") || !object.has("objective")) throw new JsonParseException("A score component needs a least a name and an objective");
                //     ScoreComponent scoreComponent = new ScoreComponent(GsonHelper.getAsString((JsonObject)object, "name"), GsonHelper.getAsString((JsonObject)object, "objective"));
                if score_json.get("name").is_none() || score_json.get("objective").is_none() {
                    return Err(de::Error::missing_field(
                        "A score component needs at least a name and an objective",
                    ));
                }
                // TODO
                return Err(de::Error::custom(
                    "score text components aren't yet supported",
                ));
                // component = ScoreComponent
            } else if json.get("selector").is_some() {
                //     } else if (jsonObject.has("selector")) {
                //         object = this.parseSeparator(type, jsonDeserializationContext, jsonObject);
                //         SelectorComponent selectorComponent = new SelectorComponent(GsonHelper.getAsString(jsonObject, "selector"), (Optional<Component>)object);

                return Err(de::Error::custom(
                    "selector text components aren't yet supported",
                ));
            //     } else if (jsonObject.has("keybind")) {
            //         KeybindComponent keybindComponent = new KeybindComponent(GsonHelper.getAsString(jsonObject, "keybind"));
            } else if json.get("keybind").is_some() {
                return Err(de::Error::custom(
                    "keybind text components aren't yet supported",
                ));
            } else {
                //     } else {
                //         if (!jsonObject.has("nbt")) throw new JsonParseException("Don't know how to turn " + jsonElement + " into a Component");
                if json.get("nbt").is_none() {
                    return Err(de::Error::custom(
                        format!("Don't know how to turn {} into a Component", json).as_str(),
                    ));
                }
                //         object = GsonHelper.getAsString(jsonObject, "nbt");
                let _nbt = json.get("nbt").unwrap().to_string();
                //         Optional<Component> optional = this.parseSeparator(type, jsonDeserializationContext, jsonObject);
                let _separator = Component::parse_separator(&json).map_err(de::Error::custom)?;

                let _interpret = match json.get("interpret") {
                    Some(v) => v.as_bool().ok_or(Some(false)).unwrap(),
                    None => false,
                };
                //         boolean bl = GsonHelper.getAsBoolean(jsonObject, "interpret", false);
                //         if (jsonObject.has("block")) {
                if json.get("block").is_some() {}
                return Err(de::Error::custom(
                    "nbt text components aren't yet supported",
                ));
                //             NbtComponent.BlockNbtComponent blockNbtComponent = new NbtComponent.BlockNbtComponent((String)object, bl, GsonHelper.getAsString(jsonObject, "block"), optional);
                //         } else if (jsonObject.has("entity")) {
                //             NbtComponent.EntityNbtComponent entityNbtComponent = new NbtComponent.EntityNbtComponent((String)object, bl, GsonHelper.getAsString(jsonObject, "entity"), optional);
                //         } else {
                //             if (!jsonObject.has("storage")) throw new JsonParseException("Don't know how to turn " + jsonElement + " into a Component");
                //             NbtComponent.StorageNbtComponent storageNbtComponent = new NbtComponent.StorageNbtComponent((String)object, bl, new ResourceLocation(GsonHelper.getAsString(jsonObject, "storage")), optional);
                //         }
                //     }
            }
            //     if (jsonObject.has("extra")) {
            //         object = GsonHelper.getAsJsonArray(jsonObject, "extra");
            //         if (object.size() <= 0) throw new JsonParseException("Unexpected empty array of components");
            //         for (int i = 0; i < object.size(); ++i) {
            //             var5_17.append(this.deserialize(object.get(i), type, jsonDeserializationContext));
            //         }
            //     }
            //     var5_17.setStyle((Style)jsonDeserializationContext.deserialize(jsonElement, Style.class));
            //     return var5_17;
            // }
            if json.get("extra").is_some() {
                let extra = match json.get("extra").unwrap().as_array() {
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
                format!("Don't know how to turn {} into a Component", json).as_str(),
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
