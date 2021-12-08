use serde_json;

use crate::{
    base_component::BaseComponent, text_component::TextComponent,
    translatable_component::TranslatableComponent,
};

pub struct Component {}

// public static class Serializer
pub impl Component {
    pub fn new(json: serde_json::Value) -> Self {
        let component: BaseComponent;
        // if it's primitive, make it a text component
        if !json.is_array() && !json.is_object() {
            return TextComponent::new(json.as_str().unwrap());
        }

        // if it's an object, do things with { text } and stuff
        if json.is_object() {
            // if it has text,
            if json.get("text").is_some() {
                let text = json.get("text").unwrap().to_string();
            }
        } else if json.get("translate").is_some() {
            let translate = json.get("translate").unwrap().to_string();
        } else if json.get("with").is_some() {
            let with = json.get("with").unwrap().as_array().unwrap();
            let mut with_array = Vec::with_capacity(with.len());
            for i in 0..with.len() {
                with_array.push(Component::new(with[i].clone()).deserialize(with[i].clone()));
            }
            let mut translatable_component = TranslatableComponent::new(translate, with_array);
        }

        Component {}
    }
}
