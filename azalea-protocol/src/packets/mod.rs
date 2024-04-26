pub mod common;
pub mod configuration;
pub mod game;
pub mod handshaking;
pub mod login;
pub mod status;

use crate::read::ReadPacketError;
use azalea_buf::{BufReadError, McBufVarReadable, McBufVarWritable, McBufWritable};
use std::io::{Cursor, Write};

// TODO: rename the packet files to just like clientbound_add_entity instead of
// clientbound_add_entity_packet

pub const PROTOCOL_VERSION: i32 = 766;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConnectionProtocol {
    Handshake = -1,
    Game = 0,
    Status = 1,
    Login = 2,
    Configuration = 3,
}

impl ConnectionProtocol {
    #[must_use]
    pub fn from_i32(i: i32) -> Option<Self> {
        match i {
            -1 => Some(ConnectionProtocol::Handshake),
            0 => Some(ConnectionProtocol::Game),
            1 => Some(ConnectionProtocol::Status),
            2 => Some(ConnectionProtocol::Login),
            3 => Some(ConnectionProtocol::Configuration),
            _ => None,
        }
    }
}

/// An enum of packets for a certain protocol
pub trait ProtocolPacket
where
    Self: Sized,
{
    fn id(&self) -> u32;

    /// Read a packet by its id, `ConnectionProtocol`, and flow
    fn read(id: u32, buf: &mut Cursor<&[u8]>) -> Result<Self, Box<ReadPacketError>>;

    fn write(&self, buf: &mut impl Write) -> Result<(), std::io::Error>;
}

/*impl azalea_buf::McBufReadable for ConnectionProtocol {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let id = i32::var_read_from(buf)?;
        ConnectionProtocol::from_i32(id).ok_or(BufReadError::UnexpectedEnumVariant { id })
    }
}

impl McBufWritable for ConnectionProtocol {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        (*self as i32).var_write_into(buf)
    }
}*/

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClientIntention {
    Status = 1,
    Login = 2,
    Transfer = 3
}

impl TryFrom<i32> for ClientIntention {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ClientIntention::Status),
            2 => Ok(ClientIntention::Login),
            3 => Ok(ClientIntention::Transfer),
            _ => Err(()),
        }
    }
}

impl Into<ConnectionProtocol> for ClientIntention {
    fn into(self) -> ConnectionProtocol {
        match self {
            ClientIntention::Status => ConnectionProtocol::Status,
            ClientIntention::Login | ClientIntention::Transfer => ConnectionProtocol::Login,
        }
    }
}

impl azalea_buf::McBufReadable for ClientIntention {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let id = i32::var_read_from(buf)?;
        id.try_into().map_err(|_| BufReadError::UnexpectedEnumVariant { id })
    }
}

impl McBufWritable for ClientIntention {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        (*self as i32).var_write_into(buf)
    }
}