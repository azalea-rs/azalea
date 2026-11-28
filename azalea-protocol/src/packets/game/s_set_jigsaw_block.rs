use std::{
    io,
    io::{Cursor, Write},
};

use azalea_buf::{AzBuf, BufReadError, BufReadErrorRepr};
use azalea_core::{identifier::Identifier, position::BlockPos};
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(AzBuf, Clone, Debug, PartialEq, ServerboundGamePacket)]
pub struct ServerboundSetJigsawBlock {
    pub pos: BlockPos,
    pub name: Identifier,
    pub target: Identifier,
    pub pool: Identifier,
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

impl AzBuf for JointType {
    fn azalea_read(ctx: azalea_buf::Ctx, buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let name = String::azalea_read(ctx, buf)?;
        match name.as_str() {
            "rollable" => Ok(JointType::Rollable),
            "aligned" => Ok(JointType::Aligned),
            _ => Err(BufReadErrorRepr::UnexpectedStringEnumVariant { id: name.into() }.into()),
        }
    }
    fn azalea_write(&self, ctx: azalea_buf::Ctx, buf: &mut impl Write) -> io::Result<()> {
        match self {
            JointType::Rollable => "rollable".to_owned().azalea_write(ctx, buf)?,
            JointType::Aligned => "aligned".to_owned().azalea_write(ctx, buf)?,
        };
        Ok(())
    }
}
