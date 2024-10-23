use std::io::Cursor;

use azalea_buf::BufReadError;
use azalea_buf::{McBufReadable, McBufWritable};
use azalea_core::bitset::FixedBitSet;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, ServerboundGamePacket)]
pub struct ServerboundPlayerInputPacket {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub shift: bool,
    pub sprint: bool,
}

impl McBufReadable for ServerboundPlayerInputPacket {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let set = FixedBitSet::<7>::read_from(buf)?;
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

impl McBufWritable for ServerboundPlayerInputPacket {
    fn write_into(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        let mut set = FixedBitSet::<7>::new();
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
        set.write_into(buf)
    }
}
