use azalea_codec::{Codec, types::StringLiteral};
#[cfg(feature = "simdnbt")]
use simdnbt::owned::NbtTag;

#[derive(Clone, Debug, PartialEq, Codec)]
#[codec(tag = "action")]
#[codec(default_snake_case_key)]
pub enum ClickEvent {
    #[codec(allow_unknown_keys)]
    OpenUrl { url: String },
    #[codec(allow_unknown_keys)]
    OpenFile { path: String },
    #[codec(allow_unknown_keys)]
    RunCommand { command: String },
    #[codec(allow_unknown_keys)]
    SuggestCommand { command: String },
    // TODO: this uses Dialog.CODEC
    #[codec(allow_unknown_keys)]
    ShowDialog {},
    #[codec(allow_unknown_keys)]
    ChangePage { page: i32 },
    #[codec(allow_unknown_keys)]
    CopyToClipboard { value: String },
    #[codec(allow_unknown_keys)]
    Custom {
        id: String,
        #[cfg(feature = "simdnbt")]
        payload: NbtTag,
    },
    #[codec(fallback)]
    Unknown,
}
