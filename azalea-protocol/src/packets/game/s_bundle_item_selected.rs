use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(AzBuf, Clone, Debug, PartialEq, ServerboundGamePacket)]
pub struct ServerboundBundleItemSelected {
    #[var]
    pub slot_id: i32,
    #[var]
    pub selected_item_index: u32,
}
