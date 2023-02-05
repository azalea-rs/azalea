use azalea_buf::{BufReadError, McBufReadable, McBufVarReadable, McBufVarWritable, McBufWritable};
use azalea_core::BlockPos;
use azalea_protocol_macros::ClientboundGamePacket;
use std::io::{Cursor, Write};

#[derive(Clone, Debug, PartialEq, ClientboundGamePacket)]
pub struct ClientboundExplodePacket {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub power: f32,
    pub to_blow: Vec<BlockPos>,
    pub knockback_x: f32,
    pub knockback_y: f32,
    pub knockback_z: f32,
}

impl McBufReadable for ClientboundExplodePacket {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let x = f32::read_from(buf)?;
        let y = f32::read_from(buf)?;
        let z = f32::read_from(buf)?;
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

        Ok(Self {
            x,
            y,
            z,
            power,
            to_blow,
            knockback_x,
            knockback_y,
            knockback_z,
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
        };
        let mut buf = Vec::new();
        packet.write_into(&mut buf).unwrap();
        let packet2 = ClientboundExplodePacket::read_from(&mut Cursor::new(&buf)).unwrap();
        assert_eq!(packet, packet2);
    }
}
