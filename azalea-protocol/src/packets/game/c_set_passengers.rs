use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSetPassengers {
    #[var]
    pub vehicle: u32,
    #[var]
    pub passengers: Vec<u32>,
}
