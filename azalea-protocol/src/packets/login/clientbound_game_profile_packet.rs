use super::LoginPacket;
use crate::mc_buf::{Readable, Writable};
use azalea_auth::game_profile::GameProfile;
use azalea_core::serializable_uuid::SerializableUuid;
use tokio::io::BufReader;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct ClientboundGameProfilePacket {
    pub game_profile: GameProfile,
}

impl ClientboundGameProfilePacket {
    pub fn get(self) -> LoginPacket {
        LoginPacket::ClientboundGameProfilePacket(self)
    }

    pub fn write(&self, buf: &mut Vec<u8>) {
        for n in self.game_profile.uuid.to_int_array() {
            buf.write_int(n as i32).unwrap();
        }
        buf.write_utf(self.game_profile.name.as_str()).unwrap();
    }

    pub async fn read<T: tokio::io::AsyncRead + std::marker::Unpin + std::marker::Send>(
        buf: &mut BufReader<T>,
    ) -> Result<LoginPacket, String> {
        let uuid = Uuid::from_int_array([
            buf.read_int().await? as u32,
            buf.read_int().await? as u32,
            buf.read_int().await? as u32,
            buf.read_int().await? as u32,
        ]);
        let name = buf.read_utf_with_len(16).await?;
        Ok(ClientboundGameProfilePacket {
            game_profile: GameProfile::new(uuid, name),
        }
        .get())
    }
}
