use super::clientbound_sound_packet::SoundSource;
use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSoundEntityPacket {
    pub sound: azalea_registry::SoundEvent,
    pub source: SoundSource,
    #[var]
    pub id: u32,
    pub volume: f32,
    pub pitch: f32,
    pub seed: u64,
}
