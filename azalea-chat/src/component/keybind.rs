use azalea_codec::Codec;

use crate::component::{base::BaseComponent, text::TextComponent};

#[derive(Clone, Debug, Default, PartialEq, Codec)]
pub struct KeybindComponent {
    pub keybind: String,
    #[codec(flatten)]
    pub base: BaseComponent,
}

impl KeybindComponent {
    pub fn into_text_component(self) -> TextComponent {
        TextComponent::default().with_base(self.base)
    }
}
