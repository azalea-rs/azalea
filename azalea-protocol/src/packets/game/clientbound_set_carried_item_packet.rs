use azalea_buf::McBuf;
use packet_macros::ClientboundGamePacket;

/// Sent to change the player's slot selection.
#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSetCarriedItemPacket {
    pub slot: u8,
}
