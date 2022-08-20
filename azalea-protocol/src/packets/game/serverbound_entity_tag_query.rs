use azalea_buf::McBuf;
use packet_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundEntityTagQuery {
    #[var]
    pub transaction_id: u32,
    #[var]
    pub entity_id: u32,
}
