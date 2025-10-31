use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

/// Unused in vanilla.
#[derive(Clone, Debug, AzBuf, PartialEq, Eq, ClientboundGamePacket)]
pub struct ClientboundPlayerCombatEnter;
