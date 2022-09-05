use azalea_buf::McBuf;
use azalea_core::Slot;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundMerchantOffersPacket {
    #[var]
    pub container_id: u32,
    pub offers: Vec<MerchantOffer>,
    #[var]
    pub villager_level: u32,
    #[var]
    pub villager_xp: u32,
    pub show_progress: bool,
    pub can_restock: bool,
}

#[derive(Clone, Debug, McBuf)]
pub struct MerchantOffer {
    pub base_cost_a: Slot,
    pub result: Slot,
    pub cost_b: Slot,
    pub out_of_stock: bool,
    pub uses: u32,
    pub max_uses: u32,
    pub xp: u32,
    pub special_price_diff: i32,
    pub price_multiplier: f32,
    pub demand: u32,
}
