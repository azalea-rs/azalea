use std::io::Cursor;

use azalea_buf::{AzBuf, AzaleaRead, AzaleaWrite};
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ServerboundGamePacket;

use crate::packets::BufReadError;

#[derive(Clone, Debug, ServerboundGamePacket)]
pub struct ServerboundSeenAdvancements {
    pub action: Action,
    pub tab: Option<ResourceLocation>,
}

#[derive(AzBuf, Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    OpenedTab = 0,
    ClosedScreen = 1,
}

impl AzaleaRead for ServerboundSeenAdvancements {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let action = Action::azalea_read(buf)?;
        let tab = if action == Action::OpenedTab {
            Some(ResourceLocation::azalea_read(buf)?)
        } else {
            None
        };
        Ok(Self { action, tab })
    }
}

impl AzaleaWrite for ServerboundSeenAdvancements {
    fn azalea_write(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        self.action.azalea_write(buf)?;
        if let Some(tab) = &self.tab {
            tab.azalea_write(buf)?;
        }
        Ok(())
    }
}
