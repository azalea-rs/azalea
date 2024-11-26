use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundSelectBundleItem {
    #[var]
    pub slot_id: i32,
    #[var]
    pub selected_item_index: u32,
}
