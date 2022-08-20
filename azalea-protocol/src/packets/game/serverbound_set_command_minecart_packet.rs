use azalea_buf::McBuf;
use packet_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundSetCommandMinecartPacket {
    #[var]
    pub entity: u32,
    pub command: String,
    pub track_output: bool,
}
