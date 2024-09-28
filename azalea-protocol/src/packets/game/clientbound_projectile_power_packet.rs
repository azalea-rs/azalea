use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundProjectilePowerPacket {
    pub id: u32,
    pub acceleration_power: f64,
}
