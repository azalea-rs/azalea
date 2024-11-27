use std::io::Cursor;
use std::io::Write;

use azalea_buf::AzBuf;
use azalea_buf::AzaleaRead;
use azalea_core::position::BlockPos;
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ServerboundGamePacket;

use crate::packets::AzaleaWrite;
use crate::packets::BufReadError;

#[derive(Clone, Debug, AzBuf, ServerboundGamePacket)]
pub struct ServerboundSetJigsawBlock {
    pub pos: BlockPos,
    pub name: ResourceLocation,
    pub target: ResourceLocation,
    pub pool: ResourceLocation,
    pub final_state: String,
    pub joint: String,
    #[var]
    pub selection_priority: i32,
    #[var]
    pub placement_priority: i32,
}

pub enum JointType {
    Rollable,
    Aligned,
}

impl AzaleaRead for JointType {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let name = String::azalea_read(buf)?;
        match name.as_str() {
            "rollable" => Ok(JointType::Rollable),
            "aligned" => Ok(JointType::Aligned),
            _ => Err(BufReadError::UnexpectedStringEnumVariant { id: name }),
        }
    }
}

impl AzaleaWrite for JointType {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match self {
            JointType::Rollable => "rollable".to_string().azalea_write(buf)?,
            JointType::Aligned => "aligned".to_string().azalea_write(buf)?,
        };
        Ok(())
    }
}
