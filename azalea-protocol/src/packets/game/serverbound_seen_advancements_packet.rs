use crate::packets::BufReadError;
use azalea_buf::{McBuf, McBufReadable, McBufWritable};
use azalea_core::ResourceLocation;
use azalea_protocol_macros::ServerboundGamePacket;
use std::io::Cursor;

#[derive(Clone, Debug, ServerboundGamePacket)]
pub struct ServerboundSeenAdvancementsPacket {
    pub action: Action,
    pub tab: Option<ResourceLocation>,
}

#[derive(McBuf, Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    OpenedTab = 0,
    ClosedScreen = 1,
}

impl McBufReadable for ServerboundSeenAdvancementsPacket {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let action = Action::read_from(buf)?;
        let tab = if action == Action::OpenedTab {
            Some(ResourceLocation::read_from(buf)?)
        } else {
            None
        };
        Ok(Self { action, tab })
    }
}

impl McBufWritable for ServerboundSeenAdvancementsPacket {
    fn write_into(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        self.action.write_into(buf)?;
        if let Some(tab) = &self.tab {
            tab.write_into(buf)?;
        }
        Ok(())
    }
}
