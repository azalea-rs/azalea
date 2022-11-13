use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundTakeItemEntityPacket {
    #[var]
    pub item_id: u32,
    #[var]
    pub player_id: u32,
    #[var]
    pub amount: u32,
}
