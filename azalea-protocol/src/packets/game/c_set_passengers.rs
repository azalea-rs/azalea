use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundSetPassengers {
    #[var]
    pub vehicle: u32,
    #[var]
    pub passengers: Vec<u32>,
}
