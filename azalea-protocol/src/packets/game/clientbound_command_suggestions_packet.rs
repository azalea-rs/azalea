use std::io::{Read, Write};

use azalea_brigadier::context::StringRange;
use azalea_buf::{
    BufReadError, McBuf, McBufReadable, McBufVarReadable, McBufVarWritable, McBufWritable,
};
use packet_macros::ClientboundGamePacket;

#[derive(Clone, Debug, ClientboundGamePacket)]
pub struct ClientboundCommandSuggestionsPacket {
    #[var]
    pub id: u32,
    // pub suggestions: Suggestions,
}

impl McBufReadable for ClientboundCommandSuggestionsPacket {
    fn read_from(buf: &mut impl Read) -> Result<Self, BufReadError> {
        // let id = u32::var_read_from(buf)?;
        // let start = u32::var_read_from(buf)? as usize;
        // let length = u32::var_read_from(buf)? as usize;
        // let stringrange = StringRange::between(start, start + length);
        todo!("Suggestions aren't implemented in azalea-brigadier yet")
    }
}

impl McBufWritable for ClientboundCommandSuggestionsPacket {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        todo!()
    }
}
