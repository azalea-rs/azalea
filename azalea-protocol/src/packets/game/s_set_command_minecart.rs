use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, AzBuf, ServerboundGamePacket)]
pub struct ServerboundSetCommandMinecart {
    #[var]
    pub entity: u32,
    pub command: String,
    pub track_output: bool,
}
