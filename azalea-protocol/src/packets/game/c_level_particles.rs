use azalea_buf::AzBuf;
use azalea_entity::particle::Particle;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundLevelParticles {
    pub override_limiter: bool,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub x_dist: f32,
    pub y_dist: f32,
    pub z_dist: f32,
    pub max_speed: f32,
    pub count: u32,
    pub particle: Particle,
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use azalea_buf::AzaleaRead;

    use super::*;

    #[test]
    fn test_c_level_particles_packet() {
        let slice = &[
            0, 64, 139, 10, 0, 0, 0, 0, 0, 192, 26, 0, 0, 0, 0, 0, 0, 64, 144, 58, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 13, 63, 128, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 63, 128, 0, 0,
        ][..];
        let mut bytes = Cursor::new(slice);

        let _packet = ClientboundLevelParticles::azalea_read(&mut bytes).unwrap();
        assert_eq!(bytes.position(), slice.len() as u64);
    }
}
