use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::Holder;
use simdnbt::owned::Nbt;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundShowDialog {
    pub dialog: Holder<azalea_registry::Dialog, Nbt>,
}
