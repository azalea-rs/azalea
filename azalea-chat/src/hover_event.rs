use azalea_codec::{Codec, types::StringLiteral};

use crate::FormattedText;

#[derive(Clone, Debug, PartialEq, Codec)]
pub enum HoverEvent {
    #[codec(allow_unknown_keys)]
    ShowText {
        action: StringLiteral<"show_text">,
        value: Box<FormattedText>,
    },
    // TODO
    #[codec(allow_unknown_keys)]
    ShowItem {
        action: StringLiteral<"show_item">,
        // item: ItemStack,
    },
    #[codec(allow_unknown_keys)]
    ShowEntity {
        action: StringLiteral<"show_entity">,
        id: i32,
        // uuid: Uuid,
        name: Box<FormattedText>,
    },
    Unknown(azalea_codec::types::Drain),
}
