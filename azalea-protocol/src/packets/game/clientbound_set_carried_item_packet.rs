use azalea_buf::McBuf;
use packet_macros::GamePacket;

/// Sent to change the player's slot selection.
#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundSetCarriedItemPacket {
    pub slot: u8,
}
