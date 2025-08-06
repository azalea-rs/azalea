use serde::Serialize;

use crate::FormattedText;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case", tag = "action")]
pub enum HoverEvent {
    ShowText {
        value: Box<FormattedText>,
    },
    // TODO
    ShowItem {
        // item: ItemStack,
    },
    ShowEntity {
        id: i32,
        // uuid: Uuid,
        name: Box<FormattedText>,
    },
}
