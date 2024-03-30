use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundMerchantOffersPacket {
    #[var]
    pub container_id: u32,
    #[var]
    pub villager_level: u32,
    #[var]
    pub villager_xp: u32,
    pub show_progress: bool,
    pub can_restock: bool,
}
