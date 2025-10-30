use serde::Serialize;
#[cfg(feature = "simdnbt")]
use simdnbt::owned::NbtCompound;

use crate::FormattedText;

#[derive(Clone, Debug, PartialEq, Serialize)]
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

#[cfg(feature = "simdnbt")]
impl simdnbt::Serialize for HoverEvent {
    fn to_compound(self) -> NbtCompound {
        let mut compound = NbtCompound::new();
        let mut action = |s: &str| {
            compound.insert("action", s);
        };
        match self {
            HoverEvent::ShowText { value } => {
                action("show_text");
                compound.insert("value", value.to_compound());
            }
            HoverEvent::ShowItem { .. } => {
                action("show_item");
            }
            HoverEvent::ShowEntity { id, name } => {
                action("show_entity");
                compound.insert("id", id);
                // compound.insert("uuid", uuid.to_string());
                compound.insert("name", name.to_compound());
            }
        }
        compound
    }
}
