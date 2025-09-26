use azalea_buf::AzBuf;
use azalea_core::sound::CustomSound;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::SoundEvent;
use azalea_world::MinecraftEntityId;

use super::c_sound::SoundSource;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundSoundEntity {
    pub sound: azalea_registry::Holder<SoundEvent, CustomSound>,
    pub source: SoundSource,
    #[var]
    pub id: MinecraftEntityId,
    pub volume: f32,
    pub pitch: f32,
    pub seed: u64,
}
