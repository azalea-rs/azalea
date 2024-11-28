use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

/// Unused in vanilla.
#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundPlayerCombatEnter;
