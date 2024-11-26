use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundAcceptTeleportation {
    #[var]
    pub id: u32,
}
