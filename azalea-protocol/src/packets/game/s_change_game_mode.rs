use azalea_buf::AzBuf;
use azalea_core::game_type::GameMode;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, AzBuf, PartialEq, ServerboundGamePacket)]
pub struct ServerboundChangeGameMode {
    pub mode: GameMode,
}
