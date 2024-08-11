use azalea_buf::McBuf;
use azalea_entity::particle::Particle;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundLevelParticlesPacket {
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

    use azalea_buf::McBufReadable;

    use super::*;

    #[test]
    fn test_clientbound_level_particles_packet() {
        let slice = &[
            0, 64, 139, 10, 0, 0, 0, 0, 0, 192, 26, 0, 0, 0, 0, 0, 0, 64, 144, 58, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 13, 63, 128, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 63, 128, 0, 0,
        ][..];
        let mut bytes = Cursor::new(slice);

        let _packet = ClientboundLevelParticlesPacket::read_from(&mut bytes).unwrap();
        assert_eq!(bytes.position(), slice.len() as u64);
    }
}
