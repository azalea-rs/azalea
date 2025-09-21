use azalea_buf::AzBuf;
use azalea_chat::FormattedText;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_world::MinecraftEntityId;

/// Used to send a respawn screen.
#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundPlayerCombatKill {
    #[var]
    pub player_id: MinecraftEntityId,
    pub message: FormattedText,
}
