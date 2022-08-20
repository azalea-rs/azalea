use azalea_buf::McBuf;
use packet_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSetSimulationDistancePacket {
    #[var]
    pub simulation_distance: u32,
}
