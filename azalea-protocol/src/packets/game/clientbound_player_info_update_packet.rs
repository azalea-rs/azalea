use std::{
    collections::HashMap,
    io::{Cursor, Write},
};

use azalea_auth::game_profile::{GameProfile, ProfilePropertyValue};
use azalea_buf::{
    BufReadError, McBuf, McBufReadable, McBufVarReadable, McBufVarWritable, McBufWritable,
};
use azalea_chat::FormattedText;
use azalea_core::{bitset::FixedBitSet, game_type::GameMode};
use azalea_protocol_macros::ClientboundGamePacket;
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
    pub list_order: i32,
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
#[derive(Clone, Debug, McBuf)]
pub struct UpdateListOrderAction {
    #[var]
    pub list_order: i32,
}

impl McBufReadable for ClientboundPlayerInfoUpdatePacket {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let actions = ActionEnumSet::read_from(buf)?;
        let mut entries = Vec::new();

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
            if actions.update_list_order {
                let action = UpdateListOrderAction::read_from(buf)?;
                entry.list_order = action.list_order;
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
    pub update_list_order: bool,
}

impl McBufReadable for ActionEnumSet {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let set = FixedBitSet::<7>::read_from(buf)?;
        Ok(ActionEnumSet {
            add_player: set.index(0),
            initialize_chat: set.index(1),
            update_game_mode: set.index(2),
            update_listed: set.index(3),
            update_latency: set.index(4),
            update_display_name: set.index(5),
            update_list_order: set.index(6),
        })
    }
}

impl McBufWritable for ActionEnumSet {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let mut set = FixedBitSet::<7>::new();
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
        if self.update_list_order {
            set.set(6);
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
            update_list_order: true,
        };
        let mut buf = Vec::new();
        data.write_into(&mut buf).unwrap();
        let mut data_cursor: Cursor<&[u8]> = Cursor::new(&buf);
        let read_data = ActionEnumSet::read_from(&mut data_cursor).unwrap();
        assert_eq!(read_data, data);
    }

    #[test]
    fn read_player_info_update_packet() {
        // from wynncraft
        let mut bytes = Cursor::new(
            &[
                63, 1, 196, 217, 99, 243, 221, 101, 79, 183, 167, 88, 48, 71, 25, 49, 5, 142, 5,
                74, 66, 76, 80, 78, 1, 8, 116, 101, 120, 116, 117, 114, 101, 115, 152, 3, 101, 119,
                111, 103, 73, 67, 74, 48, 97, 87, 49, 108, 99, 51, 82, 104, 98, 88, 65, 105, 73,
                68, 111, 103, 77, 84, 99, 119, 77, 106, 99, 49, 78, 106, 89, 48, 77, 68, 81, 120,
                78, 105, 119, 75, 73, 67, 65, 105, 99, 72, 74, 118, 90, 109, 108, 115, 90, 85, 108,
                107, 73, 105, 65, 54, 73, 67, 74, 106, 78, 71, 81, 53, 78, 106, 78, 109, 77, 50,
                82, 107, 78, 106, 85, 48, 90, 109, 73, 51, 89, 84, 99, 49, 79, 68, 77, 119, 78, 68,
                99, 120, 79, 84, 77, 120, 77, 68, 85, 52, 90, 83, 73, 115, 67, 105, 65, 103, 73,
                110, 66, 121, 98, 50, 90, 112, 98, 71, 86, 79, 89, 87, 49, 108, 73, 105, 65, 54,
                73, 67, 74, 75, 81, 107, 120, 81, 84, 105, 73, 115, 67, 105, 65, 103, 73, 110, 78,
                112, 90, 50, 53, 104, 100, 72, 86, 121, 90, 86, 74, 108, 99, 88, 86, 112, 99, 109,
                86, 107, 73, 105, 65, 54, 73, 72, 82, 121, 100, 87, 85, 115, 67, 105, 65, 103, 73,
                110, 82, 108, 101, 72, 82, 49, 99, 109, 86, 122, 73, 105, 65, 54, 73, 72, 115, 75,
                73, 67, 65, 103, 73, 67, 74, 84, 83, 48, 108, 79, 73, 105, 65, 54, 73, 72, 115, 75,
                73, 67, 65, 103, 73, 67, 65, 103, 73, 110, 86, 121, 98, 67, 73, 103, 79, 105, 65,
                105, 97, 72, 82, 48, 99, 68, 111, 118, 76, 51, 82, 108, 101, 72, 82, 49, 99, 109,
                86, 122, 76, 109, 49, 112, 98, 109, 86, 106, 99, 109, 70, 109, 100, 67, 53, 117,
                90, 88, 81, 118, 100, 71, 86, 52, 100, 72, 86, 121, 90, 83, 56, 48, 79, 68, 107,
                120, 89, 84, 107, 50, 89, 84, 74, 107, 77, 84, 103, 120, 78, 84, 69, 49, 79, 68,
                107, 52, 78, 68, 89, 119, 77, 68, 82, 106, 77, 106, 100, 104, 78, 50, 86, 106, 77,
                106, 85, 53, 77, 87, 77, 120, 79, 68, 66, 108, 77, 84, 70, 109, 77, 122, 104, 107,
                89, 122, 69, 52, 77, 84, 74, 109, 77, 106, 100, 104, 77, 71, 82, 108, 78, 50, 69,
                120, 77, 87, 85, 120, 73, 103, 111, 103, 73, 67, 65, 103, 102, 81, 111, 103, 73,
                72, 48, 75, 102, 81, 61, 61, 1, 172, 5, 117, 69, 67, 88, 54, 83, 104, 55, 67, 100,
                87, 49, 77, 99, 78, 88, 122, 72, 73, 105, 90, 86, 43, 103, 111, 121, 47, 120, 53,
                102, 51, 65, 113, 119, 50, 115, 102, 114, 104, 106, 67, 118, 67, 102, 97, 54, 67,
                112, 55, 88, 116, 109, 103, 118, 113, 73, 114, 122, 100, 85, 72, 90, 102, 79, 100,
                100, 112, 109, 87, 70, 110, 70, 119, 97, 85, 109, 97, 76, 106, 86, 102, 121, 88,
                119, 115, 76, 48, 78, 108, 118, 98, 56, 78, 104, 121, 115, 113, 87, 47, 104, 75,
                120, 101, 86, 117, 90, 68, 71, 43, 102, 54, 98, 99, 98, 81, 113, 76, 79, 54, 83,
                66, 88, 111, 81, 74, 85, 104, 74, 66, 90, 102, 88, 78, 53, 51, 100, 102, 80, 98,
                75, 89, 81, 54, 68, 77, 57, 87, 102, 113, 81, 76, 100, 55, 121, 117, 119, 90, 81,
                68, 55, 120, 48, 54, 118, 102, 105, 72, 121, 48, 110, 87, 50, 99, 68, 111, 72, 101,
                71, 102, 72, 67, 53, 104, 52, 112, 84, 109, 65, 101, 100, 101, 109, 116, 48, 67,
                72, 113, 86, 54, 76, 67, 77, 89, 118, 101, 110, 84, 88, 68, 83, 81, 107, 82, 43,
                50, 53, 74, 76, 120, 101, 98, 74, 105, 98, 108, 54, 88, 106, 73, 118, 88, 120, 105,
                87, 68, 121, 85, 49, 65, 43, 121, 48, 79, 104, 53, 89, 115, 116, 121, 86, 116, 106,
                107, 76, 113, 67, 56, 85, 57, 118, 86, 110, 87, 65, 102, 111, 43, 52, 104, 78, 43,
                79, 51, 122, 108, 72, 117, 84, 50, 87, 76, 86, 121, 98, 43, 88, 72, 100, 67, 111,
                111, 88, 75, 82, 75, 83, 86, 71, 101, 122, 103, 75, 78, 47, 53, 65, 53, 67, 119,
                78, 112, 82, 87, 98, 81, 55, 109, 90, 47, 108, 51, 57, 84, 114, 100, 84, 99, 54,
                121, 79, 88, 73, 48, 56, 83, 101, 73, 54, 68, 118, 118, 50, 55, 78, 66, 112, 107,
                47, 97, 72, 119, 65, 49, 116, 105, 78, 108, 55, 122, 49, 103, 97, 79, 107, 113,
                107, 116, 54, 120, 85, 116, 70, 84, 85, 122, 72, 71, 97, 107, 69, 118, 105, 76, 72,
                120, 67, 99, 106, 98, 121, 88, 111, 76, 71, 101, 101, 50, 57, 81, 84, 73, 102, 99,
                97, 69, 56, 104, 108, 110, 73, 97, 74, 111, 115, 72, 117, 57, 116, 100, 54, 52,
                119, 74, 88, 74, 115, 69, 78, 114, 121, 69, 56, 70, 53, 52, 52, 116, 114, 84, 54,
                105, 112, 122, 73, 119, 43, 118, 120, 112, 76, 121, 88, 65, 87, 116, 103, 83, 113,
                76, 108, 107, 121, 78, 50, 77, 115, 57, 74, 89, 110, 100, 79, 111, 90, 57, 77, 53,
                84, 49, 87, 112, 75, 70, 97, 52, 55, 114, 112, 80, 106, 75, 114, 79, 107, 114, 110,
                100, 50, 97, 83, 51, 90, 86, 77, 120, 118, 79, 49, 111, 78, 47, 100, 84, 55, 116,
                77, 119, 82, 52, 109, 97, 55, 85, 73, 68, 50, 48, 84, 113, 105, 83, 75, 56, 108,
                76, 85, 100, 53, 48, 86, 119, 108, 112, 67, 116, 98, 76, 99, 71, 86, 82, 98, 78,
                84, 97, 108, 90, 83, 66, 56, 88, 65, 72, 72, 78, 100, 116, 88, 86, 50, 49, 111, 68,
                77, 116, 77, 122, 79, 104, 82, 109, 43, 57, 88, 81, 90, 79, 50, 55, 66, 69, 71, 65,
                47, 119, 117, 104, 113, 71, 108, 106, 82, 111, 76, 72, 111, 102, 98, 71, 48, 52,
                82, 55, 84, 43, 80, 99, 112, 77, 116, 65, 69, 105, 49, 100, 57, 99, 66, 90, 115,
                119, 84, 105, 107, 113, 114, 89, 49, 86, 49, 48, 106, 104, 77, 76, 118, 99, 99, 78,
                50, 109, 70, 43, 89, 86, 81, 101, 48, 90, 55, 43, 78, 100, 119, 119, 104, 121, 47,
                108, 79, 72, 81, 54, 71, 108, 122, 74, 110, 87, 122, 103, 50, 107, 61, 0, 255, 255,
                255, 255, 15, 1, 255, 255, 255, 255, 15, 1, 10, 8, 0, 4, 116, 101, 120, 116, 0, 0,
                0,
            ][..],
        );
        let _packet = ClientboundPlayerInfoUpdatePacket::read_from(&mut bytes).unwrap();
    }
}
