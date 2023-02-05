use azalea_buf::{BufReadError, McBuf, McBufReadable, McBufWritable};
use azalea_chat::{style::ChatFormatting, FormattedText};
use azalea_protocol_macros::ClientboundGamePacket;
use std::io::{Cursor, Write};

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSetPlayerTeamPacket {
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

impl McBufReadable for Method {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(match u8::read_from(buf)? {
            0 => Method::Add((Parameters::read_from(buf)?, PlayerList::read_from(buf)?)),
            1 => Method::Remove,
            2 => Method::Change(Parameters::read_from(buf)?),
            3 => Method::Join(PlayerList::read_from(buf)?),
            4 => Method::Leave(PlayerList::read_from(buf)?),
            id => return Err(BufReadError::UnexpectedEnumVariant { id: i32::from(id) }),
        })
    }
}

impl McBufWritable for Method {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match self {
            Method::Add((parameters, playerlist)) => {
                0u8.write_into(buf)?;
                parameters.write_into(buf)?;
                playerlist.write_into(buf)?;
            }
            Method::Remove => {
                1u8.write_into(buf)?;
            }
            Method::Change(parameters) => {
                2u8.write_into(buf)?;
                parameters.write_into(buf)?;
            }
            Method::Join(playerlist) => {
                3u8.write_into(buf)?;
                playerlist.write_into(buf)?;
            }
            Method::Leave(playerlist) => {
                4u8.write_into(buf)?;
                playerlist.write_into(buf)?;
            }
        }
        Ok(())
    }
}

#[derive(McBuf, Clone, Debug)]
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
