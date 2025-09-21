use azalea_buf::AzBuf;
use azalea_core::position::Vec3;
use azalea_entity::particle::Particle;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundLevelParticles {
    pub override_limiter: bool,
    pub always_show: bool,
    pub pos: Vec3,
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
        #[rustfmt::skip]
        let slice = [0, 0, 192, 159, 104, 133, 28, 126, 5, 107, 192, 59, 0, 0, 0, 0, 0, 0, 64, 140, 27, 255, 120, 249, 188, 204, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 47, 1, 251, 245, 15, 64, 29, 194, 144, 12];
        let mut bytes = Cursor::new(slice.as_slice());

        let packet = ClientboundLevelParticles::azalea_read(&mut bytes).unwrap();
        println!("{packet:?}");
        assert_eq!(bytes.position(), slice.len() as u64);
    }
}
