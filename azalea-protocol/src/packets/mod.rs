pub mod game;
pub mod handshake;
pub mod login;
pub mod status;

use azalea_buf::{McBufWritable, Readable, Writable};
use std::io::{Read, Write};

pub const PROTOCOL_VERSION: u32 = 760;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConnectionProtocol {
    Handshake = -1,
    Game = 0,
    Status = 1,
    Login = 2,
}

impl ConnectionProtocol {
    pub fn from_i32(i: i32) -> Option<Self> {
        match i {
            -1 => Some(ConnectionProtocol::Handshake),
            0 => Some(ConnectionProtocol::Game),
            1 => Some(ConnectionProtocol::Status),
            2 => Some(ConnectionProtocol::Login),
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

    /// Read a packet by its id, ConnectionProtocol, and flow
    fn read(id: u32, buf: &mut impl Read) -> Result<Self, String>;

    fn write(&self, buf: &mut impl Write) -> Result<(), std::io::Error>;
}

impl azalea_buf::McBufReadable for ConnectionProtocol {
    fn read_from(buf: &mut impl Read) -> Result<Self, String> {
        ConnectionProtocol::from_i32(buf.read_varint()?)
            .ok_or_else(|| "Invalid intention".to_string())
    }
}

impl McBufWritable for ConnectionProtocol {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        buf.write_varint(*self as i32)
    }
}
