use crate::base_component::BaseComponent;

pub struct TextComponent {
    pub text: String,
}

impl TextComponent {
    pub fn new(text: &str) -> TextComponent {
        TextComponent {
            text: text.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        self.text.clone()
    }
}
