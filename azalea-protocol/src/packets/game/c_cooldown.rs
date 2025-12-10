use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::builtin::ItemKind;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundCooldown {
    pub item: ItemKind,
    #[var]
    pub duration: u32,
}
