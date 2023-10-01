use azalea_auth::game_profile::{GameProfile, ProfilePropertyValue};
use azalea_buf::{
    BufReadError, McBuf, McBufReadable, McBufVarReadable, McBufVarWritable, McBufWritable,
};
use azalea_chat::FormattedText;
use azalea_core::{bitset::FixedBitSet, game_type::GameMode};
use azalea_protocol_macros::ClientboundGamePacket;
use std::{
    collections::HashMap,
    io::{Cursor, Write},
};
use uuid::Uuid;

use super::serverbound_chat_session_update_packet::RemoteChatSessionData;

#[derive(Clone, Debug, ClientboundGamePacket)]
pub struct ClientboundPlayerInfoUpdatePacket {
    pub actions: ActionEnumSet,
    pub entries: Vec<PlayerInfoEntry>,
}

#[derive(Clone, Debug, Default)]
pub struct PlayerInfoEntry {
    pub profile: GameProfile,
    pub listed: bool,
    pub latency: i32,
    pub game_mode: GameMode,
    pub display_name: Option<FormattedText>,
    pub chat_session: Option<RemoteChatSessionData>,
}

#[derive(Clone, Debug, McBuf)]
pub struct AddPlayerAction {
    pub name: String,
    pub properties: HashMap<String, ProfilePropertyValue>,
}
#[derive(Clone, Debug, McBuf)]
pub struct InitializeChatAction {
    pub chat_session: Option<RemoteChatSessionData>,
}
#[derive(Clone, Debug, McBuf)]
pub struct UpdateGameModeAction {
    pub game_mode: GameMode,
}
#[derive(Clone, Debug, McBuf)]
pub struct UpdateListedAction {
    pub listed: bool,
}
#[derive(Clone, Debug, McBuf)]
pub struct UpdateLatencyAction {
    #[var]
    pub latency: i32,
}
#[derive(Clone, Debug, McBuf)]
pub struct UpdateDisplayNameAction {
    pub display_name: Option<FormattedText>,
}

impl McBufReadable for ClientboundPlayerInfoUpdatePacket {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let actions = ActionEnumSet::read_from(buf)?;
        let mut entries = vec![];

        let entry_count = u32::var_read_from(buf)?;
        for _ in 0..entry_count {
            let profile_id = Uuid::read_from(buf)?;
            let mut entry = PlayerInfoEntry::default();
            entry.profile.uuid = profile_id;

            if actions.add_player {
                let action = AddPlayerAction::read_from(buf)?;
                entry.profile.name = action.name;
                entry.profile.properties = action.properties;
            }
            if actions.initialize_chat {
                let action = InitializeChatAction::read_from(buf)?;
                entry.chat_session = action.chat_session;
            }
            if actions.update_game_mode {
                let action = UpdateGameModeAction::read_from(buf)?;
                entry.game_mode = action.game_mode;
            }
            if actions.update_listed {
                let action = UpdateListedAction::read_from(buf)?;
                entry.listed = action.listed;
            }
            if actions.update_latency {
                let action = UpdateLatencyAction::read_from(buf)?;
                entry.latency = action.latency;
            }
            if actions.update_display_name {
                let action = UpdateDisplayNameAction::read_from(buf)?;
                entry.display_name = action.display_name;
            }

            entries.push(entry);
        }

        Ok(ClientboundPlayerInfoUpdatePacket { actions, entries })
    }
}

impl McBufWritable for ClientboundPlayerInfoUpdatePacket {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        self.actions.write_into(buf)?;

        (self.entries.len() as u32).var_write_into(buf)?;
        for entry in &self.entries {
            entry.profile.uuid.write_into(buf)?;

            if self.actions.add_player {
                AddPlayerAction {
                    name: entry.profile.name.clone(),
                    properties: entry.profile.properties.clone(),
                }
                .write_into(buf)?;
            }
            if self.actions.initialize_chat {
                InitializeChatAction {
                    chat_session: entry.chat_session.clone(),
                }
                .write_into(buf)?;
            }
            if self.actions.update_game_mode {
                UpdateGameModeAction {
                    game_mode: entry.game_mode,
                }
                .write_into(buf)?;
            }
            if self.actions.update_listed {
                UpdateListedAction {
                    listed: entry.listed,
                }
                .write_into(buf)?;
            }
            if self.actions.update_latency {
                UpdateLatencyAction {
                    latency: entry.latency,
                }
                .write_into(buf)?;
            }
            if self.actions.update_display_name {
                UpdateDisplayNameAction {
                    display_name: entry.display_name.clone(),
                }
                .write_into(buf)?;
            }
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionEnumSet {
    pub add_player: bool,
    pub initialize_chat: bool,
    pub update_game_mode: bool,
    pub update_listed: bool,
    pub update_latency: bool,
    pub update_display_name: bool,
}

impl McBufReadable for ActionEnumSet {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let set = FixedBitSet::<6>::read_from(buf)?;
        Ok(ActionEnumSet {
            add_player: set.index(0),
            initialize_chat: set.index(1),
            update_game_mode: set.index(2),
            update_listed: set.index(3),
            update_latency: set.index(4),
            update_display_name: set.index(5),
        })
    }
}

impl McBufWritable for ActionEnumSet {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let mut set = FixedBitSet::<6>::new();
        if self.add_player {
            set.set(0);
        }
        if self.initialize_chat {
            set.set(1);
        }
        if self.update_game_mode {
            set.set(2);
        }
        if self.update_listed {
            set.set(3);
        }
        if self.update_latency {
            set.set(4);
        }
        if self.update_display_name {
            set.set(5);
        }
        set.write_into(buf)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_enum_set() {
        let data = ActionEnumSet {
            add_player: true,
            initialize_chat: false,
            update_game_mode: true,
            update_listed: false,
            update_latency: true,
            update_display_name: false,
        };
        let mut buf = Vec::new();
        data.write_into(&mut buf).unwrap();
        let mut data_cursor: Cursor<&[u8]> = Cursor::new(&buf);
        let read_data = ActionEnumSet::read_from(&mut data_cursor).unwrap();
        assert_eq!(read_data, data);
    }
}
