use azalea_codec::{Codec, types::StringLiteral};
use azalea_registry::identifier::Identifier;

use crate::{
    FormattedText,
    component::{base::BaseComponent, common::BlockPosArgument, text::TextComponent},
};

#[derive(Clone, Debug, PartialEq, Codec)]
pub struct NbtComponent {
    // NBT_PATH_CODEC
    pub nbt: String,
    #[codec(lenient)]
    pub interpret: bool,
    #[codec(lenient)]
    pub plain: bool,
    #[codec(lenient)]
    pub separator: Option<Box<FormattedText>>,

    #[codec(flatten)]
    pub base: BaseComponent,
}

#[derive(Clone, Debug, PartialEq, Codec)]
pub enum DataSource {
    Entity {
        source: StringLiteral<"entity">,
        // TODO: EntitySelector.COMPILABLE_CODEC
        entity: String,
    },
    Block {
        source: StringLiteral<"entity">,
        block: BlockPosArgument,
    },
    Storage {
        source: StringLiteral<"entity">,
        storage: Identifier,
    },
}

impl NbtComponent {
    pub fn into_text_component(self) -> TextComponent {
        TextComponent::default().with_base(self.base)
    }
}
