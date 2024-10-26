use std::io::{Cursor, Write};

use azalea_buf::{BufReadError, McBuf, McBufReadable, McBufWritable};
use azalea_core::{
    direction::Direction,
    position::{BlockPos, Vec3},
};
use azalea_protocol_macros::ServerboundGamePacket;

use crate::packets::game::serverbound_interact_packet::InteractionHand;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundUseItemOnPacket {
    pub hand: InteractionHand,
    pub block_hit: BlockHit,
    #[var]
    pub sequence: u32,
}

#[derive(Clone, Debug)]
pub struct BlockHit {
    /// The block that we clicked.
    pub block_pos: BlockPos,
    /// The face of the block that was clicked.
    pub direction: Direction,
    /// The exact coordinates of the world where the block was clicked. In the
    /// network, this is transmitted as the difference between the location and
    /// block position.
    pub location: Vec3,
    /// Whether the player's head is inside of a block.
    pub inside: bool,
}

impl McBufWritable for BlockHit {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        self.block_pos.write_into(buf)?;
        self.direction.write_into(buf)?;
        f32::write_into(
            &((self.location.x - f64::from(self.block_pos.x)) as f32),
            buf,
        )?;
        f32::write_into(
            &((self.location.y - f64::from(self.block_pos.y)) as f32),
            buf,
        )?;
        f32::write_into(
            &((self.location.z - f64::from(self.block_pos.z)) as f32),
            buf,
        )?;
        self.inside.write_into(buf)?;
        Ok(())
    }
}

impl McBufReadable for BlockHit {
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
                x: f64::from(block_pos.x) + f64::from(cursor_x),
                y: f64::from(block_pos.y) + f64::from(cursor_y),
                z: f64::from(block_pos.z) + f64::from(cursor_z),
            },
            inside,
        })
    }
}
