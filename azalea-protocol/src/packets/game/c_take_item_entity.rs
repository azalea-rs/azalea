use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundTakeItemEntity {
    #[var]
    pub item_id: u32,
    #[var]
    pub player_id: u32,
    #[var]
    pub amount: u32,
}
