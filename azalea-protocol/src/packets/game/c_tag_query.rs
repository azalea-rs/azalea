use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use simdnbt::owned::NbtTag;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundTagQuery {
    #[var]
    pub transaction_id: u32,
    pub tag: NbtTag,
}
