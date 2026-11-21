use azalea_codec::{Codec, types::StringLiteral};
#[cfg(feature = "simdnbt")]
use simdnbt::owned::NbtTag;

#[derive(Clone, Debug, PartialEq, Codec)]
pub enum ClickEvent {
    OpenUrl {
        action: StringLiteral<"open_url">,
        url: String,
        #[codec(flatten)]
        remaining: azalea_codec::types::Drain,
    },
    OpenFile {
        action: StringLiteral<"open_file">,
        path: String,
        #[codec(flatten)]
        remaining: azalea_codec::types::Drain,
    },
    RunCommand {
        action: StringLiteral<"run_command">,
        command: String,
        #[codec(flatten)]
        remaining: azalea_codec::types::Drain,
    },
    SuggestCommand {
        action: StringLiteral<"suggest_command">,
        command: String,
        #[codec(flatten)]
        remaining: azalea_codec::types::Drain,
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
        #[codec(flatten)]
        remaining: azalea_codec::types::Drain,
    },
    CopyToClipboard {
        action: StringLiteral<"copy_to_clipboard">,
        value: String,
        #[codec(flatten)]
        remaining: azalea_codec::types::Drain,
    },
    Custom {
        action: StringLiteral<"custom">,
        id: String,
        #[cfg(feature = "simdnbt")]
        payload: NbtTag,
        #[codec(flatten)]
        remaining: azalea_codec::types::Drain,
    },
    Unknown(azalea_codec::types::Drain),
}
