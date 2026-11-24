use azalea_codec::Codec;

use crate::FormattedText;

#[derive(Clone, Debug, PartialEq, Codec)]
#[codec(tag = "action")]
#[codec(default_snake_case_key)]
pub enum HoverEvent {
    #[codec(allow_unknown_keys)]
    ShowText { value: Box<FormattedText> },
    // TODO
    #[codec(allow_unknown_keys)]
    ShowItem {
        // item: ItemStack,
    },
    #[codec(allow_unknown_keys)]
    ShowEntity {
        id: i32,
        // uuid: Uuid,
        name: Box<FormattedText>,
    },
    #[codec(fallback)]
    Unknown,
}
