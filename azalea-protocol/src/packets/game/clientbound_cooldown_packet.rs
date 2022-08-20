use azalea_buf::McBuf;
use packet_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundCooldownPacket {
    // TODO: make azalea-items or something and use that
    #[var]
    pub item: u32,
    #[var]
    pub duration: u32,
}
