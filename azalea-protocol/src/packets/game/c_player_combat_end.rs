use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

/// Unused by the client in vanilla.
#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundPlayerCombatEnd {
    #[var]
    pub duration: u32,
}
