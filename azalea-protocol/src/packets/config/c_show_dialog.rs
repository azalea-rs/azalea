use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundConfigPacket;
use simdnbt::owned::Nbt;

#[derive(AzBuf, ClientboundConfigPacket, Clone, Debug, PartialEq)]
pub struct ClientboundShowDialog {
    pub dialog: Nbt,
}
