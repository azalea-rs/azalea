use azalea_codec::{
    Codec,
    types::{Drain, StringLiteral},
};
use azalea_core::identifier::Identifier;

use crate::{
    FormattedText,
    component::{base::BaseComponent, text::TextComponent},
};

#[derive(Clone, Debug, PartialEq, Codec)]
pub struct ObjectComponent {
    pub object: ObjectInfo,
    #[codec(optional)]
    pub fallback: Option<Box<FormattedText>>,

    #[codec(flatten)]
    pub base: BaseComponent,
}

#[derive(Clone, Debug, PartialEq, Codec)]
pub enum ObjectInfo {
    Atlas {
        object: StringLiteral<"atlas">,
        #[codec(optional)]
        atlas: Option<Identifier>,
        sprite: Identifier,
    },
    Player {
        object: StringLiteral<"player">,
        // TODO
        // player: Profile, (from azalea_inventory)
        player: Drain,
        #[codec(optional)]
        hat: bool,
    },
}

impl ObjectComponent {
    pub fn into_text_component(self) -> TextComponent {
        TextComponent::default().with_base(self.base)
    }
}
