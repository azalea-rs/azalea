use std::{
    io::{Cursor, Write},
    str::FromStr,
};

use azalea_buf::{
    BufReadError, McBuf, McBufReadable, McBufVarReadable, McBufVarWritable, McBufWritable,
};
use azalea_core::{position::BlockPos, resource_location::ResourceLocation};
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::{ParticleKind, SoundEvent};

#[derive(Clone, Debug, PartialEq, ClientboundGamePacket)]
pub struct ClientboundExplodePacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub power: f32,
    pub to_blow: Vec<BlockPos>,
    pub knockback_x: f32,
    pub knockback_y: f32,
    pub knockback_z: f32,
    pub block_interaction: BlockInteraction,
    pub small_explosion_particles: ParticleKind,
    pub large_explosion_particles: ParticleKind,
    pub explosion_sound: SoundEvent,
}

#[derive(Clone, Copy, Debug, PartialEq, McBuf)]
pub enum BlockInteraction {
    Keep,
    Destroy,
    DestroyWithDecay,
    TriggerBlock,
}

impl McBufReadable for ClientboundExplodePacket {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let x = f64::read_from(buf)?;
        let y = f64::read_from(buf)?;
        let z = f64::read_from(buf)?;
        let power = f32::read_from(buf)?;

        let x_floor = x.floor() as i32;
        let y_floor = y.floor() as i32;
        let z_floor = z.floor() as i32;

        let to_blow_len = u32::var_read_from(buf)?;
        let mut to_blow = Vec::with_capacity(to_blow_len as usize);
        for _ in 0..to_blow_len {
            // the bytes are offsets from the main x y z
            let x = x_floor + i32::from(i8::read_from(buf)?);
            let y = y_floor + i32::from(i8::read_from(buf)?);
            let z = z_floor + i32::from(i8::read_from(buf)?);
            to_blow.push(BlockPos { x, y, z });
        }

        let knockback_x = f32::read_from(buf)?;
        let knockback_y = f32::read_from(buf)?;
        let knockback_z = f32::read_from(buf)?;

        let block_interaction = BlockInteraction::read_from(buf)?;
        let small_explosion_particles = ParticleKind::read_from(buf)?;
        let large_explosion_particles = ParticleKind::read_from(buf)?;

        let sound_event_resource_location = ResourceLocation::read_from(buf)?.to_string();
        let explosion_sound =
            SoundEvent::from_str(&sound_event_resource_location).map_err(|_| {
                BufReadError::UnexpectedStringEnumVariant {
                    id: sound_event_resource_location,
                }
            })?;

        Ok(Self {
            x,
            y,
            z,
            power,
            to_blow,
            knockback_x,
            knockback_y,
            knockback_z,
            block_interaction,
            small_explosion_particles,
            large_explosion_particles,
            explosion_sound,
        })
    }
}

impl McBufWritable for ClientboundExplodePacket {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        self.x.write_into(buf)?;
        self.y.write_into(buf)?;
        self.z.write_into(buf)?;
        self.power.write_into(buf)?;

        let to_blow_len = self.to_blow.len() as u32;
        to_blow_len.var_write_into(buf)?;

        let x_floor = self.x.floor() as i32;
        let y_floor = self.y.floor() as i32;
        let z_floor = self.z.floor() as i32;

        for pos in &self.to_blow {
            let x = (pos.x - x_floor) as i8;
            let y = (pos.y - y_floor) as i8;
            let z = (pos.z - z_floor) as i8;
            x.write_into(buf)?;
            y.write_into(buf)?;
            z.write_into(buf)?;
        }

        self.knockback_x.write_into(buf)?;
        self.knockback_y.write_into(buf)?;
        self.knockback_z.write_into(buf)?;

        self.block_interaction.write_into(buf)?;
        self.small_explosion_particles.write_into(buf)?;
        self.large_explosion_particles.write_into(buf)?;

        let sound_event_resource_location =
            ResourceLocation::new(&self.explosion_sound.to_string());
        sound_event_resource_location.write_into(buf)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_write() {
        let packet = ClientboundExplodePacket {
            x: 123_456.0,
            y: 789_012.0,
            z: 345_678.0,
            power: 1_000.0,
            to_blow: vec![
                BlockPos {
                    x: 123_456 + 1,
                    y: 789_012 + 2,
                    z: 345_678 - 127,
                },
                BlockPos {
                    x: 123_456 + 4,
                    y: 789_012 - 5,
                    z: 345_678 + 6,
                },
            ],
            knockback_x: 1_000.0,
            knockback_y: 2_000.0,
            knockback_z: 3_000.0,
            block_interaction: BlockInteraction::Destroy,
            small_explosion_particles: ParticleKind::Explosion,
            large_explosion_particles: ParticleKind::ExplosionEmitter,
            explosion_sound: SoundEvent::EntityGenericExplode,
        };
        let mut buf = Vec::new();
        packet.write_into(&mut buf).unwrap();
        let packet2 = ClientboundExplodePacket::read_from(&mut Cursor::new(&buf)).unwrap();
        assert_eq!(packet, packet2);
    }
}
