use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::{Holder, data::Dialog};
use simdnbt::owned::Nbt;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundShowDialog {
    pub dialog: Holder<Dialog, Nbt>,
}
