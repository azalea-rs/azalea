use std::io::{Cursor, Write};

use azalea_buf::{AzBuf, AzaleaRead, AzaleaWrite, BufReadError};
use azalea_chat::{style::ChatFormatting, FormattedText};
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundSetPlayerTeam {
    pub name: String,
    pub method: Method,
}

#[derive(Clone, Debug)]
pub enum Method {
    Add((Parameters, PlayerList)),
    Remove,
    Change(Parameters),
    Join(PlayerList),
    Leave(PlayerList),
}

impl AzaleaRead for Method {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(match u8::azalea_read(buf)? {
            0 => Method::Add((Parameters::azalea_read(buf)?, PlayerList::azalea_read(buf)?)),
            1 => Method::Remove,
            2 => Method::Change(Parameters::azalea_read(buf)?),
            3 => Method::Join(PlayerList::azalea_read(buf)?),
            4 => Method::Leave(PlayerList::azalea_read(buf)?),
            id => return Err(BufReadError::UnexpectedEnumVariant { id: i32::from(id) }),
        })
    }
}

impl AzaleaWrite for Method {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match self {
            Method::Add((parameters, playerlist)) => {
                0u8.azalea_write(buf)?;
                parameters.azalea_write(buf)?;
                playerlist.azalea_write(buf)?;
            }
            Method::Remove => {
                1u8.azalea_write(buf)?;
            }
            Method::Change(parameters) => {
                2u8.azalea_write(buf)?;
                parameters.azalea_write(buf)?;
            }
            Method::Join(playerlist) => {
                3u8.azalea_write(buf)?;
                playerlist.azalea_write(buf)?;
            }
            Method::Leave(playerlist) => {
                4u8.azalea_write(buf)?;
                playerlist.azalea_write(buf)?;
            }
        }
        Ok(())
    }
}

#[derive(AzBuf, Clone, Debug)]
pub struct Parameters {
    pub display_name: FormattedText,
    pub options: u8,
    pub nametag_visibility: String,
    pub collision_rule: String,
    pub color: ChatFormatting,
    pub player_prefix: FormattedText,
    pub player_suffix: FormattedText,
}

type PlayerList = Vec<String>;
