use azalea_codec::{Codec, types::StringLiteral};
#[cfg(feature = "simdnbt")]
use simdnbt::owned::NbtTag;

#[derive(Clone, Debug, PartialEq, Codec)]
pub enum ClickEvent {
    #[codec(allow_unknown_keys)]
    OpenUrl {
        action: StringLiteral<"open_url">,
        url: String,
    },
    #[codec(allow_unknown_keys)]
    OpenFile {
        action: StringLiteral<"open_file">,
        path: String,
    },
    #[codec(allow_unknown_keys)]
    RunCommand {
        action: StringLiteral<"run_command">,
        command: String,
    },
    #[codec(allow_unknown_keys)]
    SuggestCommand {
        action: StringLiteral<"suggest_command">,
        command: String,
    },
    // TODO: this uses Dialog.CODEC
    #[codec(allow_unknown_keys)]
    ShowDialog {
        action: StringLiteral<"show_dialog">,
    },
    #[codec(allow_unknown_keys)]
    ChangePage {
        action: StringLiteral<"change_page">,
        page: i32,
    },
    #[codec(allow_unknown_keys)]
    CopyToClipboard {
        action: StringLiteral<"copy_to_clipboard">,
        value: String,
    },
    #[codec(allow_unknown_keys)]
    Custom {
        action: StringLiteral<"custom">,
        id: String,
        #[cfg(feature = "simdnbt")]
        payload: NbtTag,
    },
    Unknown(azalea_codec::types::Drain),
}
