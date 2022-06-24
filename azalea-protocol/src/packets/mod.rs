pub mod game;
pub mod handshake;
pub mod login;
pub mod status;

use std::io::{Read, Write};

use crate::{
    connect::PacketFlow,
    mc_buf::{McBufReadable, McBufWritable, Readable, Writable},
};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

pub const PROTOCOL_VERSION: u32 = 1073741918;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub enum ConnectionProtocol {
    Handshake = -1,
    Game = 0,
    Status = 1,
    Login = 2,
}

#[derive(Clone, Debug)]
pub enum Packet {
    Game(game::GamePacket),
    Handshake(handshake::HandshakePacket),
    Login(login::LoginPacket),
    Status(Box<status::StatusPacket>),
}

/// An enum of packets for a certain protocol
pub trait ProtocolPacket
where
    Self: Sized,
{
    fn id(&self) -> u32;

    /// Read a packet by its id, ConnectionProtocol, and flow
    fn read(id: u32, flow: &PacketFlow, buf: &mut impl Read) -> Result<Self, String>;

    fn write(&self, buf: &mut impl Write) -> Result<(), std::io::Error>;
}

impl crate::mc_buf::McBufReadable for ConnectionProtocol {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        ConnectionProtocol::from_i32(buf.read_varint()?)
            .ok_or_else(|| "Invalid intention".to_string())
    }
}

impl McBufWritable for ConnectionProtocol {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        buf.write_varint(*self as i32)
    }
}
