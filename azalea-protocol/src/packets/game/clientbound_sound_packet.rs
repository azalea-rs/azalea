use azalea_protocol_macros::ClientboundGamePacket;
use azalea_buf::McBuf;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSoundPacket {
pub source: SoundSource,
#[var]
pub x: i32,
#[var]
pub y: i32,
#[var]
pub z: i32,
pub volume: f32,
pub pitch: f32,
#[var]
pub seed: u64,
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum SoundSource {
    Master=0,
    Music=1,
    Records=2,
    Weather=3,
    Blocks=4,
    Hostile=5,
    Neutral=6,
    Players=7,
    Ambient=8,
    Voice=9,
}