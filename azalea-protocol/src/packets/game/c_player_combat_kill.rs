use azalea_buf::McBuf;
use azalea_chat::FormattedText;
use azalea_protocol_macros::ClientboundGamePacket;

/// Used to send a respawn screen.
#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundPlayerCombatKill {
    #[var]
    pub player_id: u32,
    pub message: FormattedText,
}
