use crate::{base_component::BaseComponent, component::Component};

#[derive(Clone, Debug)]
pub struct TextComponent {
    pub base: BaseComponent,
    pub text: String,
}

impl<'a> TextComponent {
    pub fn new(text: String) -> Self {
        Self {
            text: text,
            base: BaseComponent::new(),
        }
    }

    pub fn to_string(&self) -> String {
        self.text.clone()
    }
}
