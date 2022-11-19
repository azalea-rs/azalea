use crate::packets::login::serverbound_hello_packet::ProfilePublicKeyData;
use azalea_auth::game_profile::ProfilePropertyValue;
use azalea_buf::{BufReadError, McBuf};
use azalea_buf::{McBufReadable, McBufWritable};
use azalea_chat::Component;
use azalea_core::GameType;
use azalea_protocol_macros::ClientboundGamePacket;
use std::collections::HashMap;
use std::io::{Cursor, Write};
use uuid::Uuid;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundPlayerInfoPacket {
    pub action: Action,
}

#[derive(Clone, Debug)]
pub enum Action {
    AddPlayer(Vec<AddPlayer>),
    UpdateGameMode(Vec<UpdateGameMode>),
    UpdateLatency(Vec<UpdateLatency>),
    UpdateDisplayName(Vec<UpdateDisplayName>),
    RemovePlayer(Vec<RemovePlayer>),
}

#[derive(Clone, Debug, McBuf)]
pub struct AddPlayer {
    pub uuid: Uuid,
    pub name: String,
    pub properties: HashMap<String, ProfilePropertyValue>,
    pub gamemode: GameType,
    #[var]
    pub latency: i32,
    pub display_name: Option<Component>,
    pub profile_public_key: Option<ProfilePublicKeyData>,
}

#[derive(Clone, Debug, McBuf)]
pub struct UpdateGameMode {
    pub uuid: Uuid,
    pub gamemode: GameType,
}

#[derive(Clone, Debug, McBuf)]
pub struct UpdateLatency {
    pub uuid: Uuid,
    #[var]
    pub latency: i32,
}

#[derive(Clone, Debug, McBuf)]
pub struct UpdateDisplayName {
    pub uuid: Uuid,
    pub display_name: Option<Component>,
}
#[derive(Clone, Debug, McBuf)]
pub struct RemovePlayer {
    pub uuid: Uuid,
}

impl McBufReadable for Action {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let id = u8::read_from(buf)?;
        Ok(match id {
            0 => Action::AddPlayer(Vec::<AddPlayer>::read_from(buf)?),
            1 => Action::UpdateGameMode(Vec::<UpdateGameMode>::read_from(buf)?),
            2 => Action::UpdateLatency(Vec::<UpdateLatency>::read_from(buf)?),
            3 => Action::UpdateDisplayName(Vec::<UpdateDisplayName>::read_from(buf)?),
            4 => Action::RemovePlayer(Vec::<RemovePlayer>::read_from(buf)?),
            _ => return Err(BufReadError::UnexpectedEnumVariant { id: id.into() }),
        })
    }
}
impl McBufWritable for Action {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let id: u8 = match self {
            Action::AddPlayer(_) => 0,
            Action::UpdateGameMode(_) => 1,
            Action::UpdateLatency(_) => 2,
            Action::UpdateDisplayName(_) => 3,
            Action::RemovePlayer(_) => 4,
        };
        id.write_into(buf)?;
        match self {
            Action::AddPlayer(players) => players.write_into(buf)?,
            Action::UpdateGameMode(players) => players.write_into(buf)?,
            Action::UpdateLatency(players) => players.write_into(buf)?,
            Action::UpdateDisplayName(players) => players.write_into(buf)?,
            Action::RemovePlayer(players) => players.write_into(buf)?,
        }
        Ok(())
    }
}
