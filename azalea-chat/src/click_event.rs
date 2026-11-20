use azalea_codec::{Codec, types::StringLiteral};
#[cfg(feature = "simdnbt")]
use simdnbt::owned::{Nbt, NbtCompound, NbtTag};

use crate::base_component::BaseComponent;

macro_rules! define_click_event_struct {
    (
        $(
            $action_name:ident : $action_variant:ident {
                $(
                    $(#[$meta:meta])* $field:ident : $type:ty
                ),*
                $(,)?
            }
        ),*
        $(,)?
    ) => {
        #[derive(Clone, Debug, PartialEq, Serialize, Codec)]
        #[serde(rename_all = "snake_case", tag = "action")]
        pub enum ClickEvent {
            $(
                $action_variant {
                    $(
                        $(#[$meta])*
                        $field: $type
                    ),*
                }
            ),*
        }
    }
}

#[derive(Clone, Debug, PartialEq, Codec)]
pub enum ClickEvent {
    OpenUrl {
        action: StringLiteral<"open_url">,
        url: String,
    },
    OpenFile {
        action: StringLiteral<"open_file">,
        path: String,
    },
    RunCommand {
        action: StringLiteral<"run_command">,
        command: String,
    },
    SuggestCommand {
        action: StringLiteral<"suggest_command">,
        command: String,
    },
    // TODO: this uses Dialog.CODEC
    ShowDialog {
        action: StringLiteral<"show_dialog">,
        #[codec(flatten)]
        remaining: azalea_codec::types::Drain,
    },
    ChangePage {
        action: StringLiteral<"change_page">,
        page: i32,
    },
    CopyToClipboard {
        action: StringLiteral<"copy_to_clipboard">,
        value: String,
    },
    Custom {
        action: StringLiteral<"custom">,
        id: String,
        #[cfg(feature = "simdnbt")]
        payload: NbtTag,
    },
}
