use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

/// Unused by the client in vanilla.
#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundPlayerCombatEnd {
    #[var]
    pub duration: u32,
}
