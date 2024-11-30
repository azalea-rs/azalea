use std::io::{Cursor, Write};

use azalea_buf::{AzBuf, AzaleaRead, AzaleaWrite, BufReadError};
use azalea_core::{
    direction::Direction,
    position::{BlockPos, Vec3},
};
use azalea_protocol_macros::ServerboundGamePacket;

use crate::packets::game::s_interact::InteractionHand;

#[derive(Clone, Debug, AzBuf, ServerboundGamePacket)]
pub struct ServerboundUseItemOn {
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
    /// Whether the player's head is inside a block.
    pub inside: bool,
    /// Whether the player's hitting the world border.
    pub world_border: bool,
}

impl AzaleaWrite for BlockHit {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        self.block_pos.azalea_write(buf)?;
        self.direction.azalea_write(buf)?;
        f32::azalea_write(
            &((self.location.x - f64::from(self.block_pos.x)) as f32),
            buf,
        )?;
        f32::azalea_write(
            &((self.location.y - f64::from(self.block_pos.y)) as f32),
            buf,
        )?;
        f32::azalea_write(
            &((self.location.z - f64::from(self.block_pos.z)) as f32),
            buf,
        )?;
        self.inside.azalea_write(buf)?;
        self.world_border.azalea_write(buf)?;
        Ok(())
    }
}

impl AzaleaRead for BlockHit {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let block_pos = BlockPos::azalea_read(buf)?;
        let direction = Direction::azalea_read(buf)?;
        let cursor_x = f32::azalea_read(buf)?;
        let cursor_y = f32::azalea_read(buf)?;
        let cursor_z = f32::azalea_read(buf)?;
        let inside = bool::azalea_read(buf)?;
        let world_border = bool::azalea_read(buf)?;
        Ok(Self {
            block_pos,
            direction,
            location: Vec3 {
                x: f64::from(block_pos.x) + f64::from(cursor_x),
                y: f64::from(block_pos.y) + f64::from(cursor_y),
                z: f64::from(block_pos.z) + f64::from(cursor_z),
            },
            inside,
            world_border,
        })
    }
}
