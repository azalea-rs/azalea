use azalea_buf::{AzBuf, AzaleaRead, AzaleaWrite};
use azalea_core::position::Vec3;
use azalea_entity::particle::Particle;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::builtin::SoundEvent;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundExplode {
    pub center: Vec3,
    pub radius: f32,
    pub block_count: i32,
    pub player_knockback: Option<Vec3>,
    pub explosion_particle: Particle,
    pub explosion_sound: SoundEvent,
    pub block_particles: Vec<Weighted<ExplosionParticleInfo>>,
}

#[derive(AzBuf, Clone, Debug, PartialEq)]
pub struct Weighted<T: AzaleaRead + AzaleaWrite> {
    pub value: T,
    #[var]
    pub weight: i32,
}

#[derive(AzBuf, Clone, Debug, PartialEq)]
pub struct ExplosionParticleInfo {
    pub particle: Particle,
    pub scaling: f32,
    pub speed: f32,
}
