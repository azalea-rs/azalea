use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, AzBuf, ServerboundGamePacket)]
pub struct ServerboundEntityTagQuery {
    #[var]
    pub transaction_id: u32,
    #[var]
    pub entity_id: u32,
}
