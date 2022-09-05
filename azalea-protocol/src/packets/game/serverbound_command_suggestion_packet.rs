use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundCommandSuggestionPacket {
    #[var]
    pub id: u32,
    pub command: String,
}
