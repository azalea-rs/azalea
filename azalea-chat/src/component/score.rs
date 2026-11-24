use azalea_codec::{Codec, types::Drain};

use crate::component::{base::BaseComponent, text::TextComponent};

#[derive(Clone, Debug, PartialEq, Codec)]
pub struct ScoreComponent {
    pub score: ScoreInner,
    #[codec(flatten)]
    pub base: BaseComponent,
}

#[derive(Clone, Debug, PartialEq, Codec)]
pub struct ScoreInner {
    pub name: ScoreName,
    pub objective: String,
}

#[derive(Clone, Debug, PartialEq, Codec)]
pub enum ScoreName {
    String(String),
    // TODO: EntitySelector.COMPILABLE_CODEC
    EntitySelector(Drain),
}

impl ScoreComponent {
    pub fn into_text_component(self) -> TextComponent {
        TextComponent::default().with_base(self.base)
    }
}
