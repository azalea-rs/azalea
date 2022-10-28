use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundEntityTagQuery {
    #[var]
    pub transaction_id: u32,
    #[var]
    pub entity_id: u32,
}
