use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, PartialEq, Eq, ClientboundGamePacket)]
pub struct ClientboundCooldown {
    pub item: azalea_registry::Item,
    #[var]
    pub duration: u32,
}
