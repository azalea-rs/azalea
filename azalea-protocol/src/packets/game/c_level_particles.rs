use azalea_buf::AzBuf;
use azalea_core::position::Vec3;
use azalea_entity::particle::Particle;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
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
        let slice = [
            0, 0, 64, 156, 51, 153, 153, 153, 153, 154, 192, 64, 140, 204, 204, 204, 204, 205, 63, 248, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 13, 255, 0, 255, 255, 63, 128, 0, 0
        ];
        let mut bytes = Cursor::new(slice.as_slice());

        let packet = ClientboundLevelParticles::azalea_read(&mut bytes).unwrap();
        println!("{packet:?}");
        assert_eq!(bytes.position(), slice.len() as u64);
    }
}
