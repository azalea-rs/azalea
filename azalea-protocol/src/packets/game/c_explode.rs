use azalea_buf::AzBuf;
use azalea_core::position::Vec3;
use azalea_entity::particle::Particle;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::SoundEvent;

#[derive(Clone, Debug, ClientboundGamePacket, AzBuf)]
pub struct ClientboundExplode {
    pub center: Vec3,
    pub knockback: Option<Vec3>,
    pub explosion_particle: Particle,
    pub explosion_sound: SoundEvent,
}
