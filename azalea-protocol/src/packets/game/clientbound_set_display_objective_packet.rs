use azalea_buf::McBuf;
use packet_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSetDisplayObjectivePacket {
    pub slot: u8,
    pub objective_name: String,
}
