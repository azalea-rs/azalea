use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, AzBuf, ServerboundGamePacket)]
pub struct ServerboundCommandSuggestion {
    #[var]
    pub id: u32,
    pub command: String,
}
