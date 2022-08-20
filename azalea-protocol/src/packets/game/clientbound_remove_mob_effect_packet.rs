use azalea_buf::McBuf;
use packet_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundRemoveMobEffectPacket {
    #[var]
    pub entity_id: u32,
    // TODO: have this use an enum
    #[var]
    pub effect: u32,
}
