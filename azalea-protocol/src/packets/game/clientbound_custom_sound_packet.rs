use azalea_buf::McBuf;
use azalea_core::ResourceLocation;
use azalea_protocol_macros::ClientboundGamePacket;

use super::clientbound_sound_packet::SoundSource;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundCustomSoundPacket {
    pub name: ResourceLocation,
    pub source: SoundSource,
    /// x position multiplied by 8
    pub x: i32,
    /// y position multiplied by 8
    pub y: i32,
    /// z position multiplied by 8
    pub z: i32,
    pub volume: f32,
    pub pitch: f32,
    pub seed: u64,
}
