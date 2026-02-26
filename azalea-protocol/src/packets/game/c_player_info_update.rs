use std::{
    io::{self, Cursor, Write},
    sync::Arc,
};

use azalea_auth::game_profile::{GameProfile, GameProfileProperties};
use azalea_buf::{AzBuf, AzBufVar, BufReadError};
use azalea_chat::FormattedText;
use azalea_core::{bitset::FixedBitSet, game_type::GameMode};
use azalea_protocol_macros::ClientboundGamePacket;
use uuid::Uuid;

use super::s_chat_session_update::RemoteChatSessionData;

#[derive(ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundPlayerInfoUpdate {
    pub actions: ActionEnumSet,
    pub entries: Vec<PlayerInfoEntry>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PlayerInfoEntry {
    pub profile: GameProfile,
    pub chat_session: Option<RemoteChatSessionData>,
    pub game_mode: GameMode,
    pub listed: bool,
    pub latency: i32,
    pub display_name: Option<Box<FormattedText>>,
    pub list_order: i32,
    pub update_hat: bool,
}

#[derive(AzBuf, Clone, Debug)]
pub struct AddPlayerAction {
    pub name: String,
    pub properties: GameProfileProperties,
}
#[derive(AzBuf, Clone, Debug)]
pub struct InitializeChatAction {
    pub chat_session: Option<RemoteChatSessionData>,
}
#[derive(AzBuf, Clone, Debug)]
pub struct UpdateGameModeAction {
    pub game_mode: GameMode,
}
#[derive(AzBuf, Clone, Debug)]
pub struct UpdateListedAction {
    pub listed: bool,
}
#[derive(AzBuf, Clone, Debug)]
pub struct UpdateLatencyAction {
    #[var]
    pub latency: i32,
}
#[derive(AzBuf, Clone, Debug)]
pub struct UpdateDisplayNameAction {
    pub display_name: Option<Box<FormattedText>>,
}
#[derive(AzBuf, Clone, Debug)]
pub struct UpdateHatAction {
    pub update_hat: bool,
}
#[derive(AzBuf, Clone, Debug)]
pub struct UpdateListOrderAction {
    #[var]
    pub list_order: i32,
}

impl AzBuf for ClientboundPlayerInfoUpdate {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let actions = ActionEnumSet::azalea_read(buf)?;
        let mut entries = Vec::new();

        let entry_count = u32::azalea_read_var(buf)?;
        for _ in 0..entry_count {
            let profile_id = Uuid::azalea_read(buf)?;
            let mut entry = PlayerInfoEntry::default();
            entry.profile.uuid = profile_id;

            if actions.add_player {
                let action = AddPlayerAction::azalea_read(buf)?;
                entry.profile.name = action.name;
                entry.profile.properties = Arc::new(action.properties);
            }
            if actions.initialize_chat {
                let action = InitializeChatAction::azalea_read(buf)?;
                entry.chat_session = action.chat_session;
            }
            if actions.update_game_mode {
                let action = UpdateGameModeAction::azalea_read(buf)?;
                entry.game_mode = action.game_mode;
            }
            if actions.update_listed {
                let action = UpdateListedAction::azalea_read(buf)?;
                entry.listed = action.listed;
            }
            if actions.update_latency {
                let action = UpdateLatencyAction::azalea_read(buf)?;
                entry.latency = action.latency;
            }
            if actions.update_display_name {
                let action = UpdateDisplayNameAction::azalea_read(buf)?;
                entry.display_name = action.display_name;
            }
            if actions.update_list_order {
                let action = UpdateListOrderAction::azalea_read(buf)?;
                entry.list_order = action.list_order;
            }
            if actions.update_hat {
                let action = UpdateHatAction::azalea_read(buf)?;
                entry.update_hat = action.update_hat;
            }

            entries.push(entry);
        }

        Ok(ClientboundPlayerInfoUpdate { actions, entries })
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        self.actions.azalea_write(buf)?;

        (self.entries.len() as u32).azalea_write_var(buf)?;
        for entry in &self.entries {
            entry.profile.uuid.azalea_write(buf)?;

            if self.actions.add_player {
                AddPlayerAction {
                    name: entry.profile.name.clone(),
                    properties: (*entry.profile.properties).clone(),
                }
                .azalea_write(buf)?;
            }
            if self.actions.initialize_chat {
                InitializeChatAction {
                    chat_session: entry.chat_session.clone(),
                }
                .azalea_write(buf)?;
            }
            if self.actions.update_game_mode {
                UpdateGameModeAction {
                    game_mode: entry.game_mode,
                }
                .azalea_write(buf)?;
            }
            if self.actions.update_listed {
                UpdateListedAction {
                    listed: entry.listed,
                }
                .azalea_write(buf)?;
            }
            if self.actions.update_latency {
                UpdateLatencyAction {
                    latency: entry.latency,
                }
                .azalea_write(buf)?;
            }
            if self.actions.update_display_name {
                UpdateDisplayNameAction {
                    display_name: entry.display_name.clone(),
                }
                .azalea_write(buf)?;
            }
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionEnumSet {
    pub add_player: bool,
    pub initialize_chat: bool,
    pub update_game_mode: bool,
    pub update_listed: bool,
    pub update_latency: bool,
    pub update_display_name: bool,
    pub update_list_order: bool,
    pub update_hat: bool,
}

impl AzBuf for ActionEnumSet {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let set = FixedBitSet::<7>::azalea_read(buf)?;
        Ok(ActionEnumSet {
            add_player: set.index(0),
            initialize_chat: set.index(1),
            update_game_mode: set.index(2),
            update_listed: set.index(3),
            update_latency: set.index(4),
            update_display_name: set.index(5),
            update_list_order: set.index(6),
            update_hat: set.index(7),
        })
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
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
        if self.update_hat {
            set.set(7);
        }
        set.azalea_write(buf)?;
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
            update_hat: false,
        };
        let mut buf = Vec::new();
        data.azalea_write(&mut buf).unwrap();
        let mut data_cursor: Cursor<&[u8]> = Cursor::new(&buf);
        let read_data = ActionEnumSet::azalea_read(&mut data_cursor).unwrap();
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
        let _packet = ClientboundPlayerInfoUpdate::azalea_read(&mut bytes).unwrap();
    }

    #[test]
    fn read_player_info_update_packet2() {
        // from donutsmp.net
        #[rustfmt::skip]
        let contents = [253, 1, 143, 67, 23, 207, 27, 226, 78, 160, 174, 99, 203, 156, 133, 34, 245, 147, 5, 65, 113, 117, 110, 49, 1, 8, 116, 101, 120, 116, 117, 114, 101, 115, 224, 3, 101, 119, 111, 103, 73, 67, 74, 48, 97, 87, 49, 108, 99, 51, 82, 104, 98, 88, 65, 105, 73, 68, 111, 103, 77, 84, 99, 51, 77, 84, 81, 122, 79, 68, 81, 51, 78, 106, 81, 53, 79, 83, 119, 75, 73, 67, 65, 105, 99, 72, 74, 118, 90, 109, 108, 115, 90, 85, 108, 107, 73, 105, 65, 54, 73, 67, 73, 52, 90, 106, 81, 122, 77, 84, 100, 106, 90, 106, 70, 105, 90, 84, 73, 48, 90, 87, 69, 119, 89, 87, 85, 50, 77, 50, 78, 105, 79, 87, 77, 52, 78, 84, 73, 121, 90, 106, 85, 53, 77, 121, 73, 115, 67, 105, 65, 103, 73, 110, 66, 121, 98, 50, 90, 112, 98, 71, 86, 79, 89, 87, 49, 108, 73, 105, 65, 54, 73, 67, 74, 66, 99, 88, 86, 117, 77, 83, 73, 115, 67, 105, 65, 103, 73, 110, 78, 112, 90, 50, 53, 104, 100, 72, 86, 121, 90, 86, 74, 108, 99, 88, 86, 112, 99, 109, 86, 107, 73, 105, 65, 54, 73, 72, 82, 121, 100, 87, 85, 115, 67, 105, 65, 103, 73, 110, 82, 108, 101, 72, 82, 49, 99, 109, 86, 122, 73, 105, 65, 54, 73, 72, 115, 75, 73, 67, 65, 103, 73, 67, 74, 84, 83, 48, 108, 79, 73, 105, 65, 54, 73, 72, 115, 75, 73, 67, 65, 103, 73, 67, 65, 103, 73, 110, 86, 121, 98, 67, 73, 103, 79, 105, 65, 105, 97, 72, 82, 48, 99, 68, 111, 118, 76, 51, 82, 108, 101, 72, 82, 49, 99, 109, 86, 122, 76, 109, 49, 112, 98, 109, 86, 106, 99, 109, 70, 109, 100, 67, 53, 117, 90, 88, 81, 118, 100, 71, 86, 52, 100, 72, 86, 121, 90, 83, 56, 50, 78, 50, 89, 53, 79, 68, 89, 120, 78, 87, 70, 107, 78, 122, 85, 119, 89, 84, 90, 107, 77, 122, 99, 52, 78, 84, 89, 53, 79, 68, 103, 48, 78, 122, 65, 49, 89, 122, 65, 49, 90, 71, 73, 49, 79, 84, 70, 109, 78, 84, 99, 52, 77, 71, 69, 119, 89, 106, 107, 120, 78, 50, 69, 52, 78, 106, 85, 49, 79, 71, 81, 48, 78, 122, 78, 104, 77, 71, 85, 122, 78, 84, 73, 120, 73, 105, 119, 75, 73, 67, 65, 103, 73, 67, 65, 103, 73, 109, 49, 108, 100, 71, 70, 107, 89, 88, 82, 104, 73, 105, 65, 54, 73, 72, 115, 75, 73, 67, 65, 103, 73, 67, 65, 103, 73, 67, 65, 105, 98, 87, 57, 107, 90, 87, 119, 105, 73, 68, 111, 103, 73, 110, 78, 115, 97, 87, 48, 105, 67, 105, 65, 103, 73, 67, 65, 103, 73, 72, 48, 75, 73, 67, 65, 103, 73, 72, 48, 75, 73, 67, 66, 57, 67, 110, 48, 61, 1, 172, 5, 76, 55, 90, 72, 79, 73, 54, 104, 121, 104, 71, 69, 116, 84, 82, 51, 87, 81, 47, 118, 49, 111, 54, 82, 112, 80, 85, 79, 80, 112, 89, 119, 72, 50, 113, 116, 74, 51, 100, 106, 71, 67, 78, 89, 89, 99, 86, 80, 102, 70, 53, 111, 83, 118, 52, 97, 72, 52, 70, 121, 74, 74, 76, 121, 122, 68, 89, 122, 115, 84, 118, 119, 47, 78, 107, 80, 43, 68, 100, 113, 81, 103, 54, 65, 70, 110, 71, 86, 76, 112, 101, 48, 118, 111, 111, 120, 119, 49, 100, 47, 84, 105, 108, 54, 106, 115, 114, 78, 69, 86, 113, 90, 88, 104, 102, 71, 48, 72, 82, 80, 105, 66, 77, 74, 81, 82, 84, 72, 68, 78, 116, 80, 74, 74, 120, 110, 69, 57, 79, 116, 119, 52, 70, 74, 78, 79, 86, 80, 119, 118, 109, 68, 97, 65, 80, 106, 89, 104, 89, 100, 88, 106, 85, 77, 70, 115, 52, 69, 106, 81, 106, 87, 56, 69, 88, 97, 114, 71, 69, 107, 52, 105, 86, 54, 106, 107, 52, 84, 105, 102, 47, 81, 115, 54, 102, 51, 101, 105, 88, 51, 76, 52, 107, 49, 99, 47, 79, 53, 43, 57, 48, 88, 103, 102, 43, 103, 70, 97, 49, 118, 102, 67, 114, 81, 83, 110, 75, 109, 68, 100, 118, 57, 116, 82, 56, 88, 105, 99, 99, 74, 114, 80, 67, 54, 81, 81, 102, 52, 102, 113, 47, 52, 101, 80, 80, 54, 102, 86, 119, 54, 88, 70, 105, 82, 79, 90, 57, 53, 70, 90, 107, 110, 73, 97, 67, 66, 117, 104, 67, 82, 47, 99, 100, 84, 74, 121, 50, 99, 55, 107, 118, 113, 67, 111, 54, 105, 120, 52, 90, 118, 66, 106, 115, 90, 83, 115, 97, 103, 100, 56, 79, 71, 50, 79, 108, 89, 53, 43, 113, 68, 86, 48, 77, 103, 101, 54, 89, 103, 77, 107, 57, 117, 116, 86, 69, 79, 97, 53, 107, 82, 82, 117, 87, 48, 65, 98, 73, 47, 70, 97, 98, 89, 53, 65, 51, 90, 56, 116, 89, 56, 79, 56, 86, 73, 69, 77, 102, 118, 83, 105, 116, 122, 107, 99, 47, 68, 78, 82, 86, 47, 112, 115, 119, 67, 89, 119, 77, 109, 99, 105, 48, 87, 98, 114, 109, 98, 85, 67, 75, 85, 106, 75, 73, 55, 111, 121, 83, 82, 76, 57, 83, 80, 84, 118, 53, 117, 52, 68, 110, 72, 78, 87, 110, 66, 66, 111, 82, 113, 49, 43, 115, 76, 48, 87, 121, 48, 65, 90, 101, 89, 83, 81, 47, 104, 56, 72, 65, 50, 114, 66, 67, 89, 103, 120, 80, 70, 107, 54, 50, 117, 97, 83, 51, 66, 97, 110, 74, 43, 82, 83, 114, 67, 102, 104, 107, 68, 114, 81, 98, 55, 51, 54, 65, 76, 50, 47, 76, 82, 102, 85, 75, 76, 53, 67, 71, 43, 53, 98, 52, 68, 53, 112, 121, 83, 50, 116, 111, 76, 50, 83, 101, 108, 49, 77, 90, 122, 102, 115, 74, 49, 97, 80, 97, 79, 49, 52, 103, 106, 55, 77, 76, 87, 106, 121, 106, 48, 51, 88, 106, 84, 69, 113, 82, 65, 68, 104, 82, 55, 47, 78, 73, 67, 75, 117, 106, 69, 50, 57, 52, 43, 80, 114, 115, 47, 55, 67, 97, 103, 87, 118, 82, 90, 119, 90, 101, 100, 56, 90, 56, 107, 98, 52, 97, 115, 54, 102, 85, 83, 112, 90, 117, 109, 70, 89, 77, 108, 84, 65, 109, 99, 57, 110, 67, 78, 102, 72, 118, 77, 56, 109, 65, 86, 70, 73, 48, 101, 83, 98, 89, 114, 49, 77, 52, 101, 120, 116, 106, 70, 107, 76, 101, 68, 47, 99, 86, 99, 66, 100, 48, 54, 89, 109, 65, 122, 50, 51, 77, 84, 49, 115, 113, 121, 89, 100, 67, 75, 52, 105, 79, 117, 99, 98, 121, 72, 111, 84, 48, 68, 117, 72, 110, 52, 76, 105, 88, 56, 82, 67, 86, 43, 70, 56, 48, 61, 0, 1, 0, 1, 10, 9, 0, 5, 101, 120, 116, 114, 97, 10, 0, 0, 0, 3, 8, 0, 5, 99, 111, 108, 111, 114, 0, 7, 35, 70, 70, 48, 48, 65, 54, 1, 0, 10, 117, 110, 100, 101, 114, 108, 105, 110, 101, 100, 0, 1, 0, 4, 98, 111, 108, 100, 0, 1, 0, 13, 115, 116, 114, 105, 107, 101, 116, 104, 114, 111, 117, 103, 104, 0, 8, 0, 4, 116, 101, 120, 116, 0, 6, 237, 160, 189, 237, 179, 185, 1, 0, 6, 105, 116, 97, 108, 105, 99, 0, 1, 0, 10, 111, 98, 102, 117, 115, 99, 97, 116, 101, 100, 0, 0, 8, 0, 5, 99, 111, 108, 111, 114, 0, 7, 35, 48, 48, 65, 54, 70, 70, 1, 0, 10, 117, 110, 100, 101, 114, 108, 105, 110, 101, 100, 0, 1, 0, 4, 98, 111, 108, 100, 0, 1, 0, 13, 115, 116, 114, 105, 107, 101, 116, 104, 114, 111, 117, 103, 104, 0, 8, 0, 4, 116, 101, 120, 116, 0, 1, 43, 1, 0, 6, 105, 116, 97, 108, 105, 99, 0, 1, 0, 10, 111, 98, 102, 117, 115, 99, 97, 116, 101, 100, 0, 0, 8, 0, 5, 99, 111, 108, 111, 114, 0, 7, 35, 65, 52, 65, 66, 66, 55, 1, 0, 10, 117, 110, 100, 101, 114, 108, 105, 110, 101, 100, 0, 1, 0, 4, 98, 111, 108, 100, 0, 1, 0, 13, 115, 116, 114, 105, 107, 101, 116, 104, 114, 111, 117, 103, 104, 0, 8, 0, 4, 116, 101, 120, 116, 0, 5, 65, 113, 117, 110, 49, 1, 0, 6, 105, 116, 97, 108, 105, 99, 0, 1, 0, 10, 111, 98, 102, 117, 115, 99, 97, 116, 101, 100, 0, 0, 8, 0, 4, 116, 101, 120, 116, 0, 0, 0, 151, 141, 6, 1];
        let mut buf = Cursor::new(contents.as_slice());
        let _packet = ClientboundPlayerInfoUpdate::azalea_read(&mut buf).unwrap();

        assert_eq!(buf.position(), contents.len() as u64);
    }

    #[test]
    fn read_player_info_update_packet3() {
        // from donutsmp.net
        #[rustfmt::skip]
        let contents = [64, 1, 203, 169, 246, 238, 217, 134, 76, 40, 144, 100, 113, 219, 69, 183, 255, 63, 145, 141, 6];
        let mut buf = Cursor::new(contents.as_slice());
        let _packet = ClientboundPlayerInfoUpdate::azalea_read(&mut buf).unwrap();

        assert_eq!(buf.position(), contents.len() as u64);
    }
}
