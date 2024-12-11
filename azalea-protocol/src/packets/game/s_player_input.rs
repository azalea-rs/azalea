use std::io::Cursor;

use azalea_buf::BufReadError;
use azalea_buf::{AzaleaRead, AzaleaWrite};
use azalea_core::bitset::FixedBitSet;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, ServerboundGamePacket)]
pub struct ServerboundPlayerInput {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub shift: bool,
    pub sprint: bool,
}

impl AzaleaRead for ServerboundPlayerInput {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let set = FixedBitSet::<{ 7_usize.div_ceil(8) }>::azalea_read(buf)?;
        Ok(Self {
            forward: set.index(0),
            backward: set.index(1),
            left: set.index(2),
            right: set.index(3),
            jump: set.index(4),
            shift: set.index(5),
            sprint: set.index(6),
        })
    }
}

impl AzaleaWrite for ServerboundPlayerInput {
    fn azalea_write(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        let mut set = FixedBitSet::<{ 7_usize.div_ceil(8) }>::new();
        if self.forward {
            set.set(0);
        }
        if self.backward {
            set.set(1);
        }
        if self.left {
            set.set(2);
        }
        if self.right {
            set.set(3);
        }
        if self.jump {
            set.set(4);
        }
        if self.shift {
            set.set(5);
        }
        if self.sprint {
            set.set(6);
        }
        set.azalea_write(buf)
    }
}
