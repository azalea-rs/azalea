pub mod common;
pub mod config;
pub mod game;
pub mod handshake;
pub mod login;
pub mod status;

use std::io::{self, Cursor, Read, Write};

use azalea_buf::{AzBuf, AzBufVar, BufReadError};

use crate::read::ReadPacketError;

/// Network NBT framed as a VarInt byte length followed by an unnamed NBT
/// compound. A length of `0` means no NBT payload is present.
///
/// This is the 26.1.2 / Minecraft 1.21.5+ `Custom Click Action` payload
/// framing used by vanilla's `FriendlyByteBuf::writeNbt` / `readNbt`
/// contract. Historically this was represented as a bare `Nbt`, which wrote
/// the compound directly and corrupted framing for non-empty payloads.
#[derive(Clone, Debug, PartialEq)]
pub struct OptionalNbt(pub simdnbt::owned::Nbt);

impl OptionalNbt {
    pub fn empty() -> Self {
        Self(simdnbt::owned::Nbt::None)
    }
}

impl From<simdnbt::owned::Nbt> for OptionalNbt {
    fn from(value: simdnbt::owned::Nbt) -> Self {
        Self(value)
    }
}

impl AzBuf for OptionalNbt {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let length = i32::azalea_read_var(buf)?;
        if length == 0 {
            Ok(OptionalNbt(simdnbt::owned::Nbt::None))
        } else if length < 0 {
            Err(BufReadError::Custom(format!(
                "OptionalNbt length was negative: {length}"
            )))
        } else {
            let mut payload = vec![0; length as usize];
            buf.read_exact(&mut payload)?;
            let mut payload_cursor = Cursor::new(payload.as_slice());
            Ok(OptionalNbt(simdnbt::owned::Nbt::azalea_read(
                &mut payload_cursor,
            )?))
        }
    }

    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        match &self.0 {
            simdnbt::owned::Nbt::Some(_) => {
                let mut payload = Vec::new();
                self.0.azalea_write(&mut payload)?;
                (payload.len() as u32).azalea_write_var(buf)?;
                buf.write_all(&payload)?;
            }
            simdnbt::owned::Nbt::None => {
                0_u32.azalea_write_var(buf)?;
            }
        }
        Ok(())
    }
}

pub const PROTOCOL_VERSION: i32 = 775;
pub const VERSION_NAME: &str = "26.1.2";

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

/// An enum of packets for a certain protocol.
pub trait ProtocolPacket
where
    Self: Sized,
{
    fn id(&self) -> u32;

    /// Returns Mojang's resource name for the packet.
    ///
    /// This doesn't include the "minecraft:" prefix, it just returns a string
    /// like `pong`.
    fn name(&self) -> &'static str;

    /// Read a packet by its ID, `ConnectionProtocol`, and flow.
    fn read(id: u32, buf: &mut Cursor<&[u8]>) -> Result<Self, Box<ReadPacketError>>;

    fn write(&self, buf: &mut impl Write) -> io::Result<()>;
}

pub trait Packet<Protocol> {
    fn into_variant(self) -> Protocol;
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ClientIntention {
    Status = 1,
    Login = 2,
    Transfer = 3,
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

impl From<ClientIntention> for ConnectionProtocol {
    fn from(intention: ClientIntention) -> Self {
        match intention {
            ClientIntention::Status => ConnectionProtocol::Status,
            ClientIntention::Login | ClientIntention::Transfer => ConnectionProtocol::Login,
        }
    }
}

impl AzBuf for ClientIntention {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let id = i32::azalea_read_var(buf)?;
        id.try_into()
            .map_err(|_| BufReadError::UnexpectedEnumVariant { id })
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        (*self as i32).azalea_write_var(buf)
    }
}
