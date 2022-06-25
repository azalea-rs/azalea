use std::io::{Read, Write};

use super::LoginPacket;
use azalea_auth::game_profile::GameProfile;
use azalea_buf::{McBufReadable, Readable, SerializableUuid, Writable};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct ClientboundGameProfilePacket {
    pub game_profile: GameProfile,
}

// TODO: add derives to GameProfile and have an impl McBufReadable/Writable for GameProfile
impl ClientboundGameProfilePacket {
    pub fn get(self) -> LoginPacket {
        LoginPacket::ClientboundGameProfilePacket(self)
    }

    pub fn write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        for n in self.game_profile.uuid.to_int_array() {
            buf.write_int(n as i32).unwrap();
        }
        buf.write_utf(self.game_profile.name.as_str()).unwrap();
        Ok(())
    }

    pub fn read(buf: &mut impl Read) -> Result<LoginPacket, String> {
        let uuid = Uuid::read_from(buf)?;
        let name = buf.read_utf_with_len(16)?;
        Ok(ClientboundGameProfilePacket {
            game_profile: GameProfile::new(uuid, name),
        }
        .get())
    }
}
