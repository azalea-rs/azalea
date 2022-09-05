// use azalea_brigadier::context::StringRange;
use azalea_buf::{
    // BufReadError, McBuf, McBufReadable, McBufVarReadable, McBufVarWritable, McBufWritable,
    BufReadError,
    McBufReadable,
    McBufWritable,
};
use azalea_protocol_macros::ClientboundGamePacket;
use std::io::{Read, Write};

#[derive(Clone, Debug, ClientboundGamePacket)]
pub struct ClientboundCommandSuggestionsPacket {
    #[var]
    pub id: u32,
    // pub suggestions: Suggestions,
}

impl McBufReadable for ClientboundCommandSuggestionsPacket {
    fn read_from(_buf: &mut impl Read) -> Result<Self, BufReadError> {
        // let id = u32::var_read_from(buf)?;
        // let start = u32::var_read_from(buf)? as usize;
        // let length = u32::var_read_from(buf)? as usize;
        // let stringrange = StringRange::between(start, start + length);
        todo!("Suggestions aren't implemented in azalea-brigadier yet")
    }
}

impl McBufWritable for ClientboundCommandSuggestionsPacket {
    fn write_into(&self, _buf: &mut impl Write) -> Result<(), std::io::Error> {
        todo!()
    }
}
