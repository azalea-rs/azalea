use azalea_brigadier::suggestion::Suggestions;
use azalea_buf::{BufReadError, McBufReadable, McBufVarReadable, McBufVarWritable, McBufWritable};
use azalea_protocol_macros::ClientboundGamePacket;
use std::io::{Cursor, Write};

#[derive(Clone, Debug, ClientboundGamePacket)]
pub struct ClientboundCommandSuggestionsPacket {
    #[var]
    pub id: u32,
    pub suggestions: Suggestions,
}

impl McBufReadable for ClientboundCommandSuggestionsPacket {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let id = u32::var_read_from(buf)?;
        let suggestions = Suggestions::read_from(buf)?;
        Ok(ClientboundCommandSuggestionsPacket { id, suggestions })
    }
}

impl McBufWritable for ClientboundCommandSuggestionsPacket {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        self.id.var_write_into(buf)?;
        self.suggestions.write_into(buf)?;
        Ok(())
    }
}
