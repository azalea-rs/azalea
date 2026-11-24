use azalea_buf::AzBuf;
use azalea_core::position::Vec3;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::registry_holder::particle::Particle;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundLevelParticles {
    pub particle: Particle,
    pub override_limiter: bool,
    pub always_show: bool,
    pub pos: Vec3,
    pub x_dist: f32,
    pub y_dist: f32,
    pub z_dist: f32,
    pub x_max_speed: f32,
    pub y_max_speed: f32,
    pub z_max_speed: f32,
    #[var]
    pub count: u32,
    pub randomization_kind: RandomizationKind,
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq)]
pub enum RandomizationKind {
    Default,
    Alternative,
    AlternativeWithSpeed,
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use azalea_buf::AzBuf;

    use super::*;

    #[test]
    fn test_read_hypixel_particles() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .init();

        #[rustfmt::skip]
        let contents = [21, 255, 128, 0, 128, 63, 128, 0, 0, 1, 1, 192, 56, 174, 132, 32, 0, 0, 0, 64, 80, 245, 194, 128, 0, 0, 0, 192, 14, 21, 162, 192, 0, 0, 0, 63, 0, 128, 129, 0, 0, 0, 0, 63, 0, 128, 129, 63, 128, 0, 0, 63, 128, 0, 0, 63, 128, 0, 0, 0, 0];
        let mut buf: Cursor<&[u8]> = Cursor::new(contents.as_slice());
        let packet = ClientboundLevelParticles::azalea_read(Default::default(), &mut buf).unwrap();
        println!("{packet:?}");

        assert_eq!(buf.position(), contents.len() as u64);
    }
}
