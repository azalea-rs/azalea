use azalea_buf::AzBuf;
use azalea_entity::particle::Particle;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundLevelParticles {
    pub override_limiter: bool,
    pub always_show: bool,
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
            0, 0, 64, 36, 19, 1, 192, 139, 224, 69, 64, 91, 192, 0, 0, 0, 0, 0, 63, 229, 66, 62,
            20, 132, 232, 141, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 25, 153, 154, 0, 0, 0, 70,
            1, 9,
        ][..];
        let mut bytes = Cursor::new(slice);

        let packet = ClientboundLevelParticles::azalea_read(&mut bytes).unwrap();
        println!("{packet:?}");
        assert_eq!(bytes.position(), slice.len() as u64);
    }
}
