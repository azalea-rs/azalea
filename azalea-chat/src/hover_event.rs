use azalea_codec::{Codec, types::StringLiteral};

use crate::FormattedText;

#[derive(Clone, Debug, PartialEq, Codec)]
pub enum HoverEvent {
    ShowText {
        action: StringLiteral<"show_text">,
        value: Box<FormattedText>,

        #[codec(flatten)]
        remaining: azalea_codec::types::Drain,
    },
    // TODO
    ShowItem {
        action: StringLiteral<"show_item">,
        // item: ItemStack,
        #[codec(flatten)]
        remaining: azalea_codec::types::Drain,
    },
    ShowEntity {
        action: StringLiteral<"show_entity">,
        id: i32,
        // uuid: Uuid,
        name: Box<FormattedText>,

        #[codec(flatten)]
        remaining: azalea_codec::types::Drain,
    },
    Unknown(azalea_codec::types::Drain),
}
