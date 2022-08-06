use azalea_buf::McBuf;
use packet_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundCommandSuggestionPacket {
    #[var]
    pub id: u32,
    pub command: String,
}
