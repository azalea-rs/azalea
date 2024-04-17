use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundCooldownPacket {
    pub item: azalea_registry::Item,
    #[var]
    pub duration: u32,
}
