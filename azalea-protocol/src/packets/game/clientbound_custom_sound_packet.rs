use azalea_buf::McBuf;
use azalea_core::ResourceLocation;
use packet_macros::ClientboundGamePacket;

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

#[derive(McBuf, Clone, Copy, Debug)]
pub enum SoundSource {
    Master = 0,
    Music = 1,
    Records = 2,
    Weather = 3,
    Blocks = 4,
    Hostile = 5,
    Neutral = 6,
    Players = 7,
    Ambient = 8,
    Voice = 9,
}
