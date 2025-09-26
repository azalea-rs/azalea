use serde::Serialize;
#[cfg(feature = "simdnbt")]
use simdnbt::owned::{Nbt, NbtCompound, NbtTag};

#[derive(Clone, Debug, PartialEq, Serialize)]
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
        #[cfg(feature = "simdnbt")]
        payload: Nbt,
    },
}

#[cfg(feature = "simdnbt")]
impl simdnbt::Serialize for ClickEvent {
    fn to_compound(self) -> NbtCompound {
        let mut compound = NbtCompound::new();
        let mut action = |s: &str| {
            compound.insert("action", s);
        };
        match self {
            ClickEvent::OpenUrl { url } => {
                action("open_url");
                compound.insert("url", url);
            }
            ClickEvent::OpenFile { path } => {
                action("open_file");
                compound.insert("path", path);
            }
            ClickEvent::RunCommand { command } => {
                action("run_command");
                compound.insert("command", command);
            }
            ClickEvent::SuggestCommand { command } => {
                action("suggest_command");
                compound.insert("command", command);
            }
            ClickEvent::ShowDialog => {
                action("show_dialog");
            }
            ClickEvent::ChangePage { page } => {
                action("change_page");
                compound.insert("page", NbtTag::Int(page));
            }
            ClickEvent::CopyToClipboard { value } => {
                action("copy_to_clipboard");
                compound.insert("value", value);
            }
            ClickEvent::Custom { id, payload } => {
                action("custom");
                compound.insert("id", id);
                compound.insert("payload", (**payload).clone());
            }
        }
        compound
    }
}
