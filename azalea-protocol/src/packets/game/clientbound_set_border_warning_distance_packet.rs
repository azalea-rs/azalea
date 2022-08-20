use azalea_buf::McBuf;
use packet_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSetBorderWarningDistancePacket {
    #[var]
    pub warning_blocks: u32,
}
