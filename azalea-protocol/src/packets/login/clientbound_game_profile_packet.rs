use std::io::{Read, Write};

use super::ClientboundLoginPacket;
use azalea_auth::game_profile::GameProfile;
use azalea_buf::{BufReadError, McBuf, McBufReadable, Readable, SerializableUuid, Writable};
use packet_macros::ClientboundLoginPacket;
use uuid::Uuid;

#[derive(Clone, Debug, McBuf, ClientboundLoginPacket)]
pub struct ClientboundGameProfilePacket {
    pub game_profile: GameProfile,
}
