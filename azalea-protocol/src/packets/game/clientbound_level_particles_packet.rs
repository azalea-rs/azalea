use std::io::{Read, Write};

use crate::mc_buf::{McBufReadable, McBufWritable, ParticleData};
use packet_macros::GamePacket;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundLevelParticlesPacket {
    pub particle_id: u32,
    pub override_limiter: bool,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub x_dist: f32,
    pub y_dist: f32,
    pub z_dist: f32,
    pub max_speed: f32,
    pub count: i32,
    pub data: ParticleData,
}

impl McBufReadable for ClientboundLevelParticlesPacket {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        let particle_id = u32::read_into(buf)?;
        let override_limiter = bool::read_into(buf)?;
        let x = f64::read_into(buf)?;
        let y = f64::read_into(buf)?;
        let z = f64::read_into(buf)?;
        let x_dist = f32::read_into(buf)?;
        let y_dist = f32::read_into(buf)?;
        let z_dist = f32::read_into(buf)?;
        let max_speed = f32::read_into(buf)?;
        let count = i32::read_into(buf)?;

        let data = ParticleData::read_from_particle_id(buf, particle_id)?;

        Ok(Self {
            particle_id,
            override_limiter,
            x,
            y,
            z,
            x_dist,
            y_dist,
            z_dist,
            max_speed,
            count,
            data,
        })
    }
}

impl McBufWritable for ClientboundLevelParticlesPacket {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        todo!();
    }
}
