use serde::Serialize;
use simdnbt::owned::Nbt;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "action")]
pub enum ClickEvent {
    OpenUrl { url: String },
    OpenFile { path: String },
    RunCommand { command: String },
    SuggestCommand { command: String },
    // TODO: this uses Dialog.CODEC
    ShowDialog,
    ChangePage { page: i32 },
    CopyToClipboard { value: String },
    Custom { id: String, payload: Nbt },
}
