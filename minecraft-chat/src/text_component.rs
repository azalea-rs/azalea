use crate::base_component::BaseComponent;

#[derive(Clone, Debug)]
pub struct TextComponent {
    pub base: BaseComponent,
    pub text: String,
}

impl<'a> TextComponent {
    pub fn new(text: String) -> Self {
        Self {
            base: BaseComponent::new(),
            text,
        }
    }

    pub fn to_string(&self) -> String {
        self.text.clone()
    }
}
