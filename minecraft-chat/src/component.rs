use std::borrow::BorrowMut;

use serde_json;

use crate::{
    base_component::BaseComponent,
    style::Style,
    text_component::TextComponent,
    translatable_component::{StringOrComponent, TranslatableComponent},
};

#[derive(Clone)]
pub enum Component {
    TextComponent(TextComponent),
    TranslatableComponent(TranslatableComponent),
}

/// A chat component
impl Component {
    pub fn new(json: &serde_json::Value) -> Result<Component, String> {
        // we create a component that we might add siblings to
        let mut component: Component;

        // if it's primitive, make it a text component
        if !json.is_array() && !json.is_object() {
            component = Component::TextComponent(TextComponent::new(
                json.as_str().unwrap_or("").to_string(),
            ));
        }
        // if it's an object, do things with { text } and stuff
        else if json.is_object() {
            if json.get("text").is_some() {
                let text = json.get("text").unwrap().as_str().unwrap_or("").to_string();
                component = Component::TextComponent(TextComponent::new(text));
            } else if json.get("translate").is_some() {
                let translate = json.get("translate").unwrap().to_string();
                if json.get("with").is_some() {
                    let with = json.get("with").unwrap().as_array().unwrap();
                    let mut with_array = Vec::with_capacity(with.len());
                    for i in 0..with.len() {
                        // if it's a string component with no styling and no siblings, just add a string to with_array
                        // otherwise add the component to the array
                        let c = Component::new(&with[i])?;
                        if let Component::TextComponent(textComponent) = c {
                            if textComponent.base.siblings.len() == 0
                                && textComponent.base.style.is_empty()
                            {
                                with_array.push(StringOrComponent::String(textComponent.text));
                                break;
                            }
                        }
                        with_array.push(StringOrComponent::Component(Component::new(&with[i])?));
                    }
                    component = Component::TranslatableComponent(TranslatableComponent::new(
                        translate, with_array,
                    ));
                } else {
                    // if it doesn't have a "with", just have the with_array be empty
                    component = Component::TranslatableComponent(TranslatableComponent::new(
                        translate,
                        Vec::new(),
                    ));
                }
            } else if json.get("score").is_some() {
                // object = GsonHelper.getAsJsonObject(jsonObject, "score");
                let score_json = json.get("score").unwrap();
                //     if (!object.has("name") || !object.has("objective")) throw new JsonParseException("A score component needs a least a name and an objective");
                //     ScoreComponent scoreComponent = new ScoreComponent(GsonHelper.getAsString((JsonObject)object, "name"), GsonHelper.getAsString((JsonObject)object, "objective"));
                if score_json.get("name").is_none() || score_json.get("objective").is_none() {
                    return Err(
                        "A score component needs at least a name and an objective".to_string()
                    );
                }
                // TODO
                return Err("score text components aren't yet supported".to_string());
                // component = ScoreComponent
            } else if json.get("selector").is_some() {
                //     } else if (jsonObject.has("selector")) {
                //         object = this.parseSeparator(type, jsonDeserializationContext, jsonObject);
                //         SelectorComponent selectorComponent = new SelectorComponent(GsonHelper.getAsString(jsonObject, "selector"), (Optional<Component>)object);

                return Err("selector text components aren't yet supported".to_string());
            //     } else if (jsonObject.has("keybind")) {
            //         KeybindComponent keybindComponent = new KeybindComponent(GsonHelper.getAsString(jsonObject, "keybind"));
            } else if json.get("keybind").is_some() {
                return Err("keybind text components aren't yet supported".to_string());
            } else {
                //     } else {
                //         if (!jsonObject.has("nbt")) throw new JsonParseException("Don't know how to turn " + jsonElement + " into a Component");
                if json.get("nbt").is_none() {
                    return Err(format!("Don't know how to turn {} into a Component", json));
                }
                //         object = GsonHelper.getAsString(jsonObject, "nbt");
                let nbt = json.get("nbt").unwrap().to_string();
                //         Optional<Component> optional = this.parseSeparator(type, jsonDeserializationContext, jsonObject);
                let separator = Component::parse_separator(json)?;

                let interpret = match json.get("interpret") {
                    Some(v) => v.as_bool().ok_or(Some(false)).unwrap(),
                    None => false,
                };
                //         boolean bl = GsonHelper.getAsBoolean(jsonObject, "interpret", false);
                //         if (jsonObject.has("block")) {
                if json.get("block").is_some() {}
                return Err("nbt text components aren't yet supported".to_string());
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
                    None => return Err("Extra isn't an array".to_string()),
                };
                if extra.len() == 0 {
                    return Err("Unexpected empty array of components".to_string());
                }
                for extra_component in extra {
                    component.append(Component::new(extra_component)?);
                }
            }

            let style = Style::deserialize(json);
            component.get_base().style = style;

            return Ok(component);
        }
        // ok so it's not an object, if it's an array deserialize every item
        if !json.is_array() {
            return Err(format!("Don't know how to turn {} into a Component", json));
        }
        let json_array = json.as_array().unwrap();
        // the first item in the array is the one that we're gonna return, the others are siblings
        let mut component = Component::new(&json_array[0])?;
        for i in 1..json_array.len() {
            component.append(Component::new(json_array.get(i).unwrap())?);
        }
        Ok(component)
    }

    pub fn get_base(&mut self) -> &mut BaseComponent {
        match self {
            Self::TextComponent(c) => &mut c.base,
            Self::TranslatableComponent(c) => &mut c.base,
        }
    }

    /// Add a component as a sibling of this one
    fn append(&mut self, sibling: Component) {
        self.get_base().siblings.push(sibling);
    }

    /// Get the "separator" component from the json
    fn parse_separator(json: &serde_json::Value) -> Result<Option<Component>, String> {
        if json.get("separator").is_some() {
            return Ok(Some(Component::new(json.get("separator").unwrap())?));
        }
        Ok(None)
    }

    /// Convert this component into an ansi string, using parent_style as the running style.
    pub fn to_ansi(&self, parent_style: Option<&mut Style>) -> String {
        // the siblings of this component
        let base;
        let component_text: String;
        let mut styled_component = String::new();
        match self {
            Self::TextComponent(c) => {
                base = &c.base;
                component_text = c.text.clone();
            }
            Self::TranslatableComponent(c) => {
                base = &c.base;
                component_text = c.key.clone();
            }
        };

        // we'll fall back to this if there's no parent style
        let default_style = &mut Style::new();

        // if it's the base style, that means we add a style reset at the end
        let is_base_style = parent_style.is_none();

        let current_style: &mut Style = parent_style.unwrap_or(default_style);

        // the old style is current_style and the new style is the base.style
        let ansi_text = current_style.compare_ansi(&base.style);

        current_style.apply(&base.style);
        println!("\nset style to {:?}", current_style);

        styled_component.push_str(&ansi_text);
        styled_component.push_str(&component_text);

        for sibling in &base.siblings {
            styled_component.push_str(&sibling.to_ansi(Some(current_style)));
        }

        if is_base_style {
            styled_component.push_str("\x1b[m");
        }

        styled_component.clone()
    }
}
