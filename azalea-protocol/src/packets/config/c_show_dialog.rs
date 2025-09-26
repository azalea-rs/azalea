use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundConfigPacket;
use simdnbt::owned::Nbt;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundConfigPacket)]
pub struct ClientboundShowDialog {
    pub dialog: Nbt,
}
