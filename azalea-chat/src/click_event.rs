use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case", tag = "action")]
pub enum ClickEvent {
    OpenUrl {
        url: String,
    },
    OpenFile {
        path: String,
    },
    RunCommand {
        command: String,
    },
    SuggestCommand {
        command: String,
    },
    // TODO: this uses Dialog.CODEC
    ShowDialog,
    ChangePage {
        page: i32,
    },
    CopyToClipboard {
        value: String,
    },
    Custom {
        id: String,
        // TODO
        // payload: Nbt,
    },
}
