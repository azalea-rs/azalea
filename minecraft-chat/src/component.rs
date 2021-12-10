use std::borrow::BorrowMut;

use serde_json;

use crate::{
    base_component::BaseComponent,
    style::Style,
    text_component::TextComponent,
    translatable_component::{StringOrComponent, TranslatableComponent},
};

#[derive(Clone, Debug)]
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
            return Ok(Component::TextComponent(TextComponent::new(
                json.as_str().unwrap_or("").to_string(),
            )));
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
                        if let Component::TextComponent(text_component) = c {
                            if text_component.base.siblings.len() == 0
                                && text_component.base.style.is_empty()
                            {
                                with_array.push(StringOrComponent::String(text_component.text));
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
            component.get_base_mut().style = style;

            return Ok(component);
        }
        // ok so it's not an object, if it's an array deserialize every item
        else if !json.is_array() {
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

    // TODO: is it possible to use a macro so this doesn't have to be duplicated?

    pub fn get_base_mut(&mut self) -> &mut BaseComponent {
        match self {
            Self::TextComponent(c) => &mut c.base,
            Self::TranslatableComponent(c) => &mut c.base,
        }
    }

    pub fn get_base(&self) -> &BaseComponent {
        match self {
            Self::TextComponent(c) => &c.base,
            Self::TranslatableComponent(c) => &c.base,
        }
    }

    /// Add a component as a sibling of this one
    fn append(&mut self, sibling: Component) {
        self.get_base_mut().siblings.push(sibling);
    }

    /// Get the "separator" component from the json
    fn parse_separator(json: &serde_json::Value) -> Result<Option<Component>, String> {
        if json.get("separator").is_some() {
            return Ok(Some(Component::new(json.get("separator").unwrap())?));
        }
        Ok(None)
    }

    /// Recursively call the function for every component in this component
    pub fn visit<F>(&self, f: &mut F) -> ()
    where
        // The closure takes an `i32` and returns an `i32`.
        F: FnMut(&Component) -> (),
    {
        f(self);
        self.get_base()
            .siblings
            .iter()
            .for_each(|s| Component::visit(s, f));
    }

    /// Convert this component into an ansi string, using parent_style as the running style.
    pub fn to_ansi(&self, _: Option<()>) -> String {
        // this contains the final string will all the ansi escape codes
        let mut built_string = String::new();
        // this style will update as we visit components
        let mut running_style = Style::new();

        self.visit(&mut |component| {
            let component_text = match component {
                Self::TextComponent(c) => &c.text,
                Self::TranslatableComponent(c) => &c.key,
            };
            let component_style = &component.get_base().style;

            let ansi_text = running_style.compare_ansi(component_style);
            built_string.push_str(&ansi_text);
            built_string.push_str(&component_text);

            running_style.apply(&component_style);
        });

        if !running_style.is_empty() {
            built_string.push_str("\x1b[m");
        }

        built_string
    }
}
