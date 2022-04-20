// i don't know the actual name of this packet, i couldn't find it in the source code

use crate::mc_buf::{McBufReadable, McBufWritable, Readable};
use async_trait::async_trait;
use packet_macros::GamePacket;
use tokio::io::AsyncRead;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundPlayerAbilitiesPacket {
    pub flags: PlayerAbilitiesFlags,
    pub flying_speed: f32,
    /// Used for the fov
    pub walking_speed: f32,
}

#[derive(Clone, Debug)]
pub struct PlayerAbilitiesFlags {
    pub invulnerable: bool,
    pub flying: bool,
    pub can_fly: bool,
    pub instant_break: bool,
}

// Difficulty
#[async_trait]
impl McBufReadable for PlayerAbilitiesFlags {
    async fn read_into<R>(buf: &mut R) -> Result<Self, String>
    where
        R: AsyncRead + std::marker::Unpin + std::marker::Send,
    {
        let byte = buf.read_byte().await?;
        Ok(PlayerAbilitiesFlags {
            invulnerable: byte & 1 != 0,
            flying: byte & 2 != 0,
            can_fly: byte & 4 != 0,
            instant_break: byte & 8 != 0,
        })
    }
}

// Difficulty
impl McBufWritable for PlayerAbilitiesFlags {
    fn write_into(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let mut byte = 0;
        if self.invulnerable {
            byte = byte | 1;
        }
        if self.flying {
            byte = byte | 2;
        }
        if self.can_fly {
            byte = byte | 4;
        }
        if self.instant_break {
            byte = byte | 8;
        }
        u8::write_into(&byte, buf)
    }
}
