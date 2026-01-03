use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::{Holder, data::Dialog};
use simdnbt::owned::Nbt;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundShowDialog {
    pub dialog: Holder<Dialog, Nbt>,
}
