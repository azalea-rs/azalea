use std::io::{self, Cursor, Write};

use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;
use azalea_registry::identifier::Identifier;

use crate::packets::BufReadError;

#[derive(Clone, Debug, PartialEq, ServerboundGamePacket)]
pub struct ServerboundSeenAdvancements {
    pub action: Action,
    pub tab: Option<Identifier>,
}

#[derive(AzBuf, Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    OpenedTab = 0,
    ClosedScreen = 1,
}

impl AzBuf for ServerboundSeenAdvancements {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let action = Action::azalea_read(buf)?;
        let tab = if action == Action::OpenedTab {
            Some(Identifier::azalea_read(buf)?)
        } else {
            None
        };
        Ok(Self { action, tab })
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        self.action.azalea_write(buf)?;
        if let Some(tab) = &self.tab {
            tab.azalea_write(buf)?;
        }
        Ok(())
    }
}
