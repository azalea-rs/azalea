use super::clientbound_sound_packet::SoundSource;
use azalea_buf::McBuf;
use packet_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSoundEntityPacket {
    // TODO: sound enum/registry
    #[var]
    pub sound: u32,
    pub source: SoundSource,
    #[var]
    pub id: u32,
    pub volume: f32,
    pub pitch: f32,
    pub seed: u64,
}
