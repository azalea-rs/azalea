use azalea_buf::AzBuf;
use azalea_core::position::Vec3;
use azalea_entity::particle::Particle;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::SoundEvent;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundExplode {
    pub center: Vec3,
    pub radius: f32,
    pub block_count: i32,
    pub player_knockback: Option<Vec3>,
    pub explosion_particle: Particle,
    pub explosion_sound: SoundEvent,
    pub block_particles: Vec<ExplosionParticleInfo>,
}

#[derive(Clone, Debug, AzBuf, PartialEq)]
pub struct ExplosionParticleInfo {
    pub particle: Particle,
    pub scaling: f32,
    pub speed: f32,
}
