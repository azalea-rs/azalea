use azalea_buf::{BufReadError, McBufReadable, McBufVarReadable, McBufVarWritable, McBufWritable};
use azalea_core::ParticleData;
use azalea_protocol_macros::ClientboundGamePacket;
use std::io::{Cursor, Write};

#[derive(Clone, Debug, ClientboundGamePacket)]
pub struct ClientboundLevelParticlesPacket {
    #[var]
    pub particle_id: u32,
    pub override_limiter: bool,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub x_dist: f32,
    pub y_dist: f32,
    pub z_dist: f32,
    pub max_speed: f32,
    pub count: u32,
    pub data: ParticleData,
}

impl McBufReadable for ClientboundLevelParticlesPacket {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let particle_id = u32::var_read_from(buf)?;
        let override_limiter = bool::read_from(buf)?;
        let x = f64::read_from(buf)?;
        let y = f64::read_from(buf)?;
        let z = f64::read_from(buf)?;
        let x_dist = f32::read_from(buf)?;
        let y_dist = f32::read_from(buf)?;
        let z_dist = f32::read_from(buf)?;
        let max_speed = f32::read_from(buf)?;
        let count = u32::read_from(buf)?;

        let data = ParticleData::read_from_id(buf, particle_id)?;

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
        self.particle_id.var_write_into(buf)?;
        self.override_limiter.write_into(buf)?;
        self.x.write_into(buf)?;
        self.y.write_into(buf)?;
        self.z.write_into(buf)?;
        self.x_dist.write_into(buf)?;
        self.y_dist.write_into(buf)?;
        self.z_dist.write_into(buf)?;
        self.max_speed.write_into(buf)?;
        self.count.write_into(buf)?;
        self.data.write_without_id(buf)?;
        Ok(())
    }
}
