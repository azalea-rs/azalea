use crate::packets::game::serverbound_interact_packet::InteractionHand;
use azalea_buf::{BufReadError, McBuf, McBufReadable, McBufWritable};
use azalea_core::{BlockPos, Direction, Vec3};
use azalea_protocol_macros::ServerboundGamePacket;
use std::io::{Cursor, Write};

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundUseItemOnPacket {
    pub hand: InteractionHand,
    pub block_hit: BlockHitResult,
    #[var]
    pub sequence: u32,
}

#[derive(Clone, Debug)]
pub struct BlockHitResult {
    pub block_pos: BlockPos,
    pub direction: Direction,
    pub location: Vec3,
    pub inside: bool,
}

impl McBufWritable for BlockHitResult {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        self.block_pos.write_into(buf)?;
        self.direction.write_into(buf)?;
        f32::write_into(&((self.location.x - (self.block_pos.x as f64)) as f32), buf)?;
        f32::write_into(&((self.location.y - (self.block_pos.y as f64)) as f32), buf)?;
        f32::write_into(&((self.location.z - (self.block_pos.z as f64)) as f32), buf)?;
        self.inside.write_into(buf)?;
        Ok(())
    }
}

impl McBufReadable for BlockHitResult {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let block_pos = BlockPos::read_from(buf)?;
        let direction = Direction::read_from(buf)?;
        let cursor_x = f32::read_from(buf)?;
        let cursor_y = f32::read_from(buf)?;
        let cursor_z = f32::read_from(buf)?;
        let inside = bool::read_from(buf)?;
        Ok(Self {
            block_pos,
            direction,
            location: Vec3 {
                x: block_pos.x as f64 + cursor_x as f64,
                y: block_pos.y as f64 + cursor_y as f64,
                z: block_pos.z as f64 + cursor_z as f64,
            },
            inside,
        })
    }
}
