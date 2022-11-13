use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

/// Unused in vanilla.
#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundPlayerCombatEnterPacket {}
