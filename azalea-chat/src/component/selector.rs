use azalea_codec::Codec;

use crate::{
    FormattedText,
    component::{base::BaseComponent, common::EntitySelector, text::TextComponent},
};

#[derive(Clone, Debug, Default, PartialEq, Codec)]
pub struct SelectorComponent {
    pub selector: EntitySelector,
    pub separator: Option<Box<FormattedText>>,
    #[codec(flatten)]
    pub base: BaseComponent,
}

impl SelectorComponent {
    pub fn into_text_component(self) -> TextComponent {
        TextComponent::default().with_base(self.base)
    }
}
