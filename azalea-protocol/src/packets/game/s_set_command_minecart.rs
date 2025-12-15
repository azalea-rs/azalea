use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(AzBuf, Clone, Debug, PartialEq, ServerboundGamePacket)]
pub struct ServerboundSetCommandMinecart {
    #[var]
    pub entity: u32,
    pub command: String,
    pub track_output: bool,
}
