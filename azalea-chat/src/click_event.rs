use serde::Serialize;
#[cfg(feature = "simdnbt")]
use simdnbt::{
    DeserializeError,
    owned::{Nbt, NbtCompound},
};

#[cfg(feature = "simdnbt")]
use crate::get_in_compound;

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
        #[derive(Clone, Debug, PartialEq, Serialize)]
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

        #[cfg(feature = "simdnbt")]
        impl simdnbt::Serialize for ClickEvent {
            fn to_compound(self) -> NbtCompound {
                let mut compound = NbtCompound::new();
                match self {
                    $(
                        Self::$action_variant { $($field),* } => {
                            compound.insert("action", stringify!($action_name));
                            $(
                                compound.insert(stringify!($field), $field);
                            )*
                        }
                    )*
                };
                compound
            }
        }

        #[cfg(feature = "simdnbt")]
        impl simdnbt::Deserialize for ClickEvent {
            fn from_compound(
                compound: simdnbt::borrow::NbtCompound,
            ) -> Result<Self, simdnbt::DeserializeError> {
                let action = get_in_compound::<String>(&compound, "action")?;
                Ok(match action.as_str() {
                    $(
                        stringify!($action_name) => Self::$action_variant {
                            $(
                                $field: get_in_compound(&compound, stringify!($field))?
                            ),*
                        },
                    )*
                    _ => return Err(DeserializeError::MismatchedFieldType(action.to_owned())),
                })
            }
        }

    }
}

define_click_event_struct! {
    open_url: OpenUrl {
        url: String,
    },
    open_file: OpenFile {
        path: String,
    },
    run_command: RunCommand {
        command: String,
    },
    suggest_command: SuggestCommand {
        command: String,
    },
    // TODO: this uses Dialog.CODEC
    show_dialog: ShowDialog {},
    change_page: ChangePage {
        page: i32,
    },
    copy_to_clipboard: CopyToClipboard {
        value: String,
    },
    custom: Custom {
        id: String,
        #[cfg(feature = "simdnbt")]
        payload: Nbt,
    },
}
