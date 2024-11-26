use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundSetCommandMinecart {
    #[var]
    pub entity: u32,
    pub command: String,
    pub track_output: bool,
}
